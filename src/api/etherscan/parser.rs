// Etherscan response parsers

use crate::api::error::{ApiError, ApiResult};
use crate::models::{EthPriceData, PriceExtractor, TokenTransfer};
use serde_json::Value;

pub struct ResponseParser;

impl ResponseParser {
    pub fn parse_token_transfers(json: &Value) -> ApiResult<Vec<TokenTransfer>> {
        let result = json
            .get("result")
            .ok_or_else(|| ApiError::ParseError("No result field".to_string()))?;

        // Handle string error messages
        if let Some(msg) = result.as_str() {
            if msg.to_lowercase().contains("no transactions") || msg.is_empty() {
                return Ok(Vec::new());
            }
            return Err(ApiError::RpcError(msg.to_string()));
        }

        // Check status
        let status = json.get("status").and_then(|v| v.as_str()).unwrap_or("0");

        if status != "1" {
            if let Some(arr) = result.as_array()
                && arr.is_empty()
            {
                return Ok(Vec::new());
            }
            let msg = json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(ApiError::RpcError(msg.to_string()));
        }

        serde_json::from_value(result.clone()).map_err(|e| ApiError::ParseError(e.to_string()))
    }

    pub fn parse_eth_price(json: &Value) -> ApiResult<(f64, f64)> {
        let status = json.get("status").and_then(|v| v.as_str()).unwrap_or("0");

        if status != "1" {
            let message = json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(ApiError::RpcError(message.to_string()));
        }

        let result = json
            .get("result")
            .ok_or_else(|| ApiError::ParseError("No result field".to_string()))?;

        let price_data: EthPriceData = serde_json::from_value(result.clone())
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        let ethusd = price_data.price_usd().map_err(ApiError::ParseError)?;
        let ethbtc = price_data.price_btc().map_err(ApiError::ParseError)?;

        Ok((ethusd, ethbtc))
    }
}
