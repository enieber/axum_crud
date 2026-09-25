use axum::{
    routing::{get},
    Router
};

use crate::handlers::tasks;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/tasks", get(tasks::list)
                        .post(tasks::create))
        .route("/tasks/{id}", get(tasks::get_one)
                        .patch(tasks::update)
                        .delete(tasks::remove))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "Ok"
}
