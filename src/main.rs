use tokio::net::TcpListener;
use std::env;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use tower_http::trace::TraceLayer;

mod models;
mod state;
mod routes;
mod handlers;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("error,tower_http=info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.unwrap();

    let state = AppState {
        db: conn,
    };

    let app = routes::app(state).layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<axum::body::Body>| {
                    // Cria um span com os dados da requisição para rastreamento
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    // Loga especificamente quando o status for 5xx
                    if response.status().is_server_error() {
                        tracing::error!(
                            status = %response.status(),
                            latency_ms = latency.as_millis(),
                            "Server error occurred"
                        );
                    }
                })
                .on_failure(|error: tower_http::classify::ServerErrorsFailureClass, latency: std::time::Duration, _span: &tracing::Span| {
                    // Loga falhas críticas (incluindo panics interceptados pelo Axum)
                    tracing::error!(
                        error = %error,
                        latency_ms = latency.as_millis(),
                        "Request failed critically"
                    );
                })
        );

    let listener = TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
