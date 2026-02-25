// Etherscan price API

use reqwest::Client;
use std::sync::OnceLock;
use crate::api::etherscan::{url, parser};
use crate::api::error::ApiResult;
use crate::cache::GLOBAL_PRICE_CACHE;

/// Shared HTTP client for price requests - reused across all calls
static SHARED_CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    SHARED_CLIENT.get_or_init(Client::new)
}

pub async fn get_eth_price(api_key: &str) -> ApiResult<(f64, f64)> {
    // Check cache first
    if let Some((price, ethbtc)) = GLOBAL_PRICE_CACHE.get().await {
        return Ok((price, ethbtc));
    }

    let client = get_client();
    let url = url::EtherscanUrlBuilder::new(api_key).eth_price();

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| crate::api::error::ApiError::HttpError(e.to_string()))?;

    let data: serde_json::Value = response.json()
        .await
        .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))?;

    let (ethusd, ethbtc) = parser::ResponseParser::parse_eth_price(&data)?;

    // Price fetched successfully (no print in TUI mode)

    // Update cache
    GLOBAL_PRICE_CACHE.set(ethusd, ethbtc).await;

    Ok((ethusd, ethbtc))
}
