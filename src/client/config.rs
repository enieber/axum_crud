use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_base_url: String,
    pub http_timeout_secs: u64,
}

impl Config {
    pub fn setup() -> Self {
        Self {
            api_base_url: env::var("API_BASE_URL").unwrap_or_else(|_| "http::/127.0.0.1:3000/".into()),
            http_timeout_secs: std::env::var("HTTP_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        }
    }
}
