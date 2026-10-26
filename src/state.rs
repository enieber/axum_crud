use reqwest::{Client, Url};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub http: Client,
    pub api_base: Url,
}
