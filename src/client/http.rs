use anyhow::Result;
use axum::http::StatusCode;
use reqwest::{Client, ClientBuilder, Url};
use std::time::Duration;

pub fn build_client(base_url: &str, timeout_sec: u64) -> Result<Client> {
    let _ = Url::parse(base_url)?;

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(timeout_sec))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(30))
        .user_agent(concat!("crud_axum/", env!("CARGO_PKG_VERSION")))
        .build()?;

    Ok(client)
}

#[derive(Debug)]
pub enum ApiError {
    Unreachable(String),
    Status(StatusCode),
    Decode(String),
    Url(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unreachable(e) => write!(f, "API inacessível: {e}"),
            ApiError::Status(s) => write!(f, "API retornou {s}"),
            ApiError::Decode(e) => write!(f, "JSON inválido: {e}"),
            ApiError::Url(e) => write!(f, "URL inválida: {e}"),
        }
    }
}
