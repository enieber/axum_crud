use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use crate::controllers::web::{tasks};

pub fn web_routes() -> Router<AppState> {
    Router::new()
        .route("/tasks",    get(tasks::tasks_page).post(tasks::create_task))
        .route("/tasks/new", get(tasks::new_task_page))
        .route("/tasks/{id}/edit", get(tasks::edit_task_page))
        .route("/tasks/{id}", post(tasks::update_task))
        .route("/tasks/{id}/delete", post(tasks::delete_task))
}
