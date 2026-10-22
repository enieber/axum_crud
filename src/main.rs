use tokio::net::TcpListener;
use std::env;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database};
use tracing_subscriber::EnvFilter;
use tower_http::trace::TraceLayer;
use dotenvy::dotenv;
use reqwest::Url;

mod models;
mod routes;
mod controllers;
mod state;
mod client;

use crate::client::config::Config;
use crate::client::http::build_client;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config_http = Config::setup();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("error,tower_http=info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut api_base = Url::parse("http://127.0.0.1:3000").unwrap();

    let db_conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&db_conn, None).await.unwrap();

    let http = build_client(&config_http.api_base_url, config_http.http_timeout_secs).unwrap();
    if !api_base.path().ends_with('/') {
        api_base.set_path(&format!("{}/", api_base.path()));
    }

    let state = AppState {
        db: db_conn,
        http,
        api_base,
    };

    let app = routes::app(state).layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
