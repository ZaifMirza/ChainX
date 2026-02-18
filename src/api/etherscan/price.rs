// Etherscan price API

use reqwest::Client;
use crate::api::etherscan::{url, parser};
use crate::api::error::ApiResult;
use crate::cache::GLOBAL_PRICE_CACHE;

pub async fn get_eth_price(api_key: &str) -> ApiResult<(f64, f64)> {
    // Check cache first
    if let Some((price, ethbtc)) = GLOBAL_PRICE_CACHE.get().await {
        return Ok((price, ethbtc));
    }

    let client = Client::new();
    let url = url::EtherscanUrlBuilder::new(api_key).eth_price();

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| crate::api::error::ApiError::HttpError(e.to_string()))?;

    let data: serde_json::Value = response.json()
        .await
        .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))?;

    let (ethusd, ethbtc) = parser::ResponseParser::parse_eth_price(&data)?;

    println!("💰 ETH Price: ${} USD | {} BTC", ethusd, ethbtc);

    // Update cache
    GLOBAL_PRICE_CACHE.set(ethusd, ethbtc).await;

    Ok((ethusd, ethbtc))
}
