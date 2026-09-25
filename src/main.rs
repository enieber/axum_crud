use tokio::net::TcpListener;
mod models;
mod state;
mod routes;
mod handlers;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = routes::app(state);

    let listener = TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
