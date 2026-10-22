use axum::{routing::{get}, Router};
use crate::state::AppState;
use crate::controllers::web::{tasks};

pub fn web_routes() -> Router<AppState> {
    Router::new()
        .route("/task",    get(tasks::tasks_page))
}
