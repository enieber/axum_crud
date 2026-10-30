use axum_crud::views::form::FormField;
use axum_crud::views::list::ListItemView;
use serde::de::DeserializeOwned;

pub trait WebConfig {
    type Model: Clone;
    type Client;
    type CreateDto: DeserializeOwned;
    type UpdateDto: DeserializeOwned;
    type Id: std::fmt::Display + std::str::FromStr + Send + Sync + 'static;

    const ROUTE_BASE: &'static str;
    const TITLE_SINGULAR: &'static str;
    const TITLE_PLURAL: &'static str;

    // Renderização
    fn build_list_item(model: &Self::Model) -> ListItemView;
    fn build_create_fields() -> Vec<FormField>;
    fn build_update_fields(model: &Self::Model) -> Vec<FormField>;

    // Validação (retorna Some("erro") se inválido, None se válido)
    fn validate_create(dto: &Self::CreateDto) -> Option<String>;
    fn validate_update(dto: &Self::UpdateDto) -> Option<String>;
}
