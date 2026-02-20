// Contract command handler with rate limiting

use crate::app::AppState;
use crate::api::{get_eth_price, etherscan::ContractClient};
use crate::utils::wei_to_eth;
use crate::error::Result;
use crate::models::{ContractDisplay};

pub struct ContractCommand;

impl ContractCommand {
    /// Check if API key is required for this command
    pub fn requires_api_key() -> bool {
        true
    }

    /// Execute for TUI - returns ContractDisplay instead of printing
    pub async fn execute_tui(state: &AppState, address: &str) -> Result<ContractDisplay> {
        // Check if API key is configured
        let api_key = state.config.etherscan_api_key.as_ref()
            .ok_or_else(|| crate::error::ExplorerError::ConfigError(
                "Etherscan API key not configured. Press 's' to set up your API key.".to_string()
            ))?;
        
        // Fetch ETH balance and price
        let (eth_balance, eth_price) = Self::fetch_balance_and_price(state, address, api_key).await
            .unwrap_or(("0x0".to_string(), 0.0));
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        // Create contract client
        let contract_client = ContractClient::new(api_key.clone());
        
        // Fetch contract info
        let contract_info = contract_client.get_contract_info(address).await.ok();
        
        // Fetch creation info
        let creation_info = contract_client.get_contract_creation(address).await.ok();
        
        // Fetch transactions
        let transactions = contract_client.get_contract_transactions(address, 5).await.unwrap_or_default();
        
        let mut display = ContractDisplay::new(address.to_string());
        display.balance_eth = balance_eth_val;
        display.usd_value = if eth_price > 0.0 {
            Some(format!("${:.2} USD", balance_usd_val))
        } else {
            None
        };
        display.transaction_count = transactions.len();
        display.recent_transactions = transactions;

        // Populate contract info if available
        if let Some(info) = contract_info {
            if !info.contract_name.is_empty() {
                display.name = Some(info.contract_name);
            }
            if !info.compiler_version.is_empty() {
                display.compiler = Some(info.compiler_version);
            }
            display.is_proxy = info.proxy == "1";
            if !info.implementation.is_empty() {
                display.implementation = Some(info.implementation);
            }
        }

        // Populate creation info if available
        if let Some(creation) = creation_info {
            display.creator = Some(creation.contract_creator);
            display.creation_transaction = Some(creation.tx_hash);
        }
        
        Ok(display)
    }
    
    async fn fetch_balance_and_price(state: &AppState, address: &str, api_key: &str) -> Result<(String, f64)> {
        let eth_balance = state.rpc_client.get_balance(address).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))?;
        
        let (eth_price, _) = get_eth_price(api_key).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch ETH price: {}", e)))?;
        
        Ok((eth_balance, eth_price))
    }
}
