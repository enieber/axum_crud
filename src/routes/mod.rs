// pub mod api;
pub mod web;
pub mod security_addons;

use axum::Router;
use utoipa::{
    Modify,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;
use crate::controllers::api::{tasks, auth};
use crate::routes::web::web_routes;
use crate::routes::security_addons::SecurityAddon;

pub fn app(state: AppState) -> Router {
    let (router, mut api) = OpenApiRouter::new()
        .nest("/api/v1", tasks::routes())
        .nest("/api/v1", auth::routes())
        .with_state(state.clone())
        .split_for_parts();

    SecurityAddon.modify(&mut api);

     let web = web_routes().with_state(state);

    router
        .merge(web)
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api),
        )
}
