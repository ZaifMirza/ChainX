// Etherscan contract API

use reqwest::Client;
use serde_json::Value;
use crate::api::error::{ApiError, ApiResult};
use crate::models::{ContractCreationInfo, ContractInfo, ContractTransaction};

pub struct ContractClient {
    api_key: String,
    client: Client,
}

impl ContractClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    /// Get contract ABI and info
    pub async fn get_contract_info(&self, address: &str) -> ApiResult<ContractInfo> {
        let url = format!(
            "https://api.etherscan.io/v2/api?chainid=1&module=contract&action=getsourcecode&address={}&apikey={}",
            address, self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::HttpError(e.to_string()))?;

        let json: Value = response.json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        // Check status
        let status = json.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("0");

        let message = json.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");

        if status != "1" && !message.to_lowercase().contains("ok") {
            // Contract might not be verified - return empty info instead of error
            return Ok(ContractInfo::default());
        }

        let result = json.get("result")
            .ok_or_else(|| ApiError::ParseError("No result field".to_string()))?;

        // Result is an array with one element
        let contracts: Vec<ContractInfo> = serde_json::from_value(result.clone())
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        contracts.into_iter()
            .next()
            .ok_or_else(|| ApiError::RpcError("No contract info found".to_string()))
    }

    /// Get contract creator and creation transaction
    pub async fn get_contract_creation(&self, address: &str) -> ApiResult<ContractCreationInfo> {
        let url = format!(
            "https://api.etherscan.io/v2/api?chainid=1&module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
            address, self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::HttpError(e.to_string()))?;

        let json: Value = response.json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        // Check status
        let status = json.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("0");

        if status != "1" {
            let message = json.get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(ApiError::RpcError(message.to_string()));
        }

        let result = json.get("result")
            .ok_or_else(|| ApiError::ParseError("No result field".to_string()))?;

        // Result is an array with one element
        let creations: Vec<ContractCreationInfo> = serde_json::from_value(result.clone())
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        creations.into_iter()
            .next()
            .ok_or_else(|| ApiError::RpcError("No contract creation info found".to_string()))
    }

    /// Get transactions for a contract (both internal and external)
    pub async fn get_contract_transactions(&self, address: &str, limit: usize) -> ApiResult<Vec<ContractTransaction>> {
        let url = format!(
            "https://api.etherscan.io/v2/api?chainid=1&module=account&action=txlist&address={}&page=1&offset={}&sort=desc&apikey={}",
            address, limit, self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::HttpError(e.to_string()))?;

        let json: Value = response.json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        // Parse result first
        let result = json.get("result")
            .ok_or_else(|| ApiError::ParseError("No result field".to_string()))?;

        // Check if result is a string (error message)
        if let Some(msg) = result.as_str() {
            if msg.to_lowercase().contains("no transactions") || msg.is_empty() {
                return Ok(vec![]);
            }
            return Err(ApiError::RpcError(msg.to_string()));
        }

        // Check status
        let status = json.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("0");

        if status != "1" {
            if let Some(arr) = result.as_array()
                && arr.is_empty()
            {
                return Ok(vec![]);
            }
            let message = json.get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(ApiError::RpcError(message.to_string()));
        }

        let transactions: Vec<ContractTransaction> = serde_json::from_value(result.clone())
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        Ok(transactions)
    }
}
