use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::state::AppState;
use crate::controllers::api::tasks;

pub fn api_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/tasks", OpenApiRouter::new()
            .routes(routes!(tasks::list))
            .routes(routes!(tasks::create))
            .routes(routes!(tasks::get_one))
            .routes(routes!(tasks::update))
            .routes(routes!(tasks::remove))
        )
        // .nest("/users", controllers::api::users::routes())
}

