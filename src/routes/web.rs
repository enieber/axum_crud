use crate::controllers::web::{auth, posts, tasks};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn web_routes() -> Router<AppState> {
    let tasks_routes = Router::new()
        .route("/", get(tasks::list_view).post(tasks::register_form))
        .route("/new", get(tasks::create_view))
        .route("/{id}/edit", get(tasks::update_view))
        .route("/{id}", post(tasks::update_form))
        .route("/{id}/delete", post(tasks::delete_form));

    let posts_routes = Router::new()
        .route("/", get(posts::list_view).post(posts::register_form))
        .route("/new", get(posts::create_view))
        .route("/{id}/edit", get(posts::update_view))
        .route("/{id}", post(posts::update_form))
        .route("/{id}/delete", post(posts::delete_form));

    let auth_routes = Router::new().route(
        "/register",
        get(auth::register_view).post(auth::register_form),
    );

    Router::new()
        .nest("/tasks", tasks_routes)
        .nest("/posts", posts_routes)
        .nest("/auth", auth_routes)
}
