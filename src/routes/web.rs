use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use crate::controllers::web::{auth, dashboard};

pub fn web_routes() -> Router<AppState> {
    Router::new()
        .route("/login",    get(auth::login_page).post(auth::login_submit))
        .route("/logout",   post(auth::logout))
        .route("/dashboard", get(dashboard::dashboard_page))
}

