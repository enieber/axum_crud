use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use crate::handlers::tasks; // Ajuste o caminho do seu handler
use crate::models::task::{Model, CreateTask, UpdateTask}; // Ajuste o caminho

#[derive(OpenApi)]
#[openapi(
    paths(
        tasks::list,
        tasks::create,
        tasks::get_one,
        tasks::update,
        tasks::remove
    ),
    components(
        schemas(Model, CreateTask, UpdateTask)
    ),
    tags(
        (name = "Tasks", description = "Task management API")
    ),
    modifiers(&SecurityAddon) // Adiciona o botão "Authorize" no Swagger
)]
pub struct ApiDoc;

// Configuração de Segurança (API Key no Header)
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization")))
            )
        }
    }
}
