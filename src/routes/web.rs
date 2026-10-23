use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use crate::controllers::web::{tasks, auth};

pub fn web_routes() -> Router<AppState> {
    let tasks_routes = Router::new()
        .route("/", get(tasks::tasks_page).post(tasks::create_task))
        .route("/new", get(tasks::new_task_page))
        .route("/{id}/edit", get(tasks::edit_task_page))
        .route("/{id}", post(tasks::update_task))
        .route("/{id}/delete", post(tasks::delete_task));

    let auth_routes = Router::new()
        .route(
            "/register",
            get(auth::register_page).post(auth::create_user),
        );

    Router::new()
        .nest("/tasks", tasks_routes)
        .nest("/auth", auth_routes)
}
