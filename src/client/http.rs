use reqwest::{Client, ClientBuilder, Url};
use std::time::Duration;
use anyhow::Result;

pub fn build_client(base_url: &str, timeout_sec: u64) -> Result<Client> {
    let _  = Url::parse(base_url)?;

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(timeout_sec))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Duration::from_secs(30))
        .user_agent(concat!("crud_axum/", env!("CARGO_PKG_VERSION")))
        .build()?;

    Ok(client)
}
