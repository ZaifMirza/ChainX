// Etherscan token transactions API

use reqwest::Client;
use crate::api::etherscan::{url, parser};
use crate::api::error::ApiResult;
use crate::models::TokenTransfer;

#[allow(dead_code)]
pub async fn get_token_transactions(api_key: &str, address: &str) -> ApiResult<Vec<TokenTransfer>> {
    let client = Client::new();
    let url = url::EtherscanUrlBuilder::new(api_key).token_transactions(address);

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| crate::api::error::ApiError::HttpError(e.to_string()))?;

    let json: serde_json::Value = response.json()
        .await
        .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))?;

    parser::ResponseParser::parse_token_transfers(&json)
}

pub struct EtherscanClient {
    api_key: String,
    client: Client,
}

impl EtherscanClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get_token_transactions(&self, address: &str) -> ApiResult<Vec<TokenTransfer>> {
        let url = url::EtherscanUrlBuilder::new(&self.api_key).token_transactions(address);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| crate::api::error::ApiError::HttpError(e.to_string()))?;

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))?;

        parser::ResponseParser::parse_token_transfers(&json)
    }
}
