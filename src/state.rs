use sea_orm::{DatabaseConnection};
use reqwest::{Client, Url};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub http: Client,
    pub api_base: Url,
}
