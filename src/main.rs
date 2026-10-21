use tokio::net::TcpListener;
use std::env;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;

mod models;
mod routes;
mod controllers;
mod state;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    let app = routes::app(state).layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
