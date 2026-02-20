// Address command handler with rate limiting

use crate::app::AppState;
use crate::api::{EtherscanClient, get_eth_price};
use crate::models::{TokenInfo, AddressDisplay, TokenBalanceDisplay};
use crate::utils::wei_to_eth;
use crate::error::Result;
use std::collections::HashSet;

pub struct AddressCommand;

impl AddressCommand {
    /// Check if API key is required for this command
    pub fn requires_api_key() -> bool {
        true
    }

    /// Execute for TUI - returns AddressDisplay instead of printing
    pub async fn execute_tui(state: &AppState, address: &str) -> Result<AddressDisplay> {
        // Check if API key is configured
        let api_key = state.config.etherscan_api_key.as_ref()
            .ok_or_else(|| crate::error::ExplorerError::ConfigError(
                "Etherscan API key not configured. Press 's' to set up your API key.".to_string()
            ))?;
        
        // Fetch ETH balance
        let eth_balance = state.rpc_client.get_balance(address).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))?;
        
        // Fetch ETH price
        let eth_price_result = get_eth_price(api_key).await;
        let eth_price = match eth_price_result {
            Ok((price, _)) => price,
            Err(_) => 0.0,
        };
        
        // Fetch token transactions
        let etherscan = EtherscanClient::new(api_key.clone());
        let transfers = etherscan.get_token_transactions(address).await.unwrap_or_default();
        
        // Calculate values
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        // Process token balances
        let mut seen: HashSet<String> = HashSet::new();
        let mut token_balances: Vec<TokenBalanceDisplay> = Vec::new();
        
        for transfer in &transfers {
            if transfer.is_suspicious() {
                continue;
            }
            if !seen.contains(&transfer.contract_address) {
                seen.insert(transfer.contract_address.clone());
                token_balances.push(TokenBalanceDisplay::from_transfer(transfer));
            }
            if token_balances.len() >= 10 {
                break;
            }
        }

        // Get transaction count
        let tx_count = state.rpc_client.get_transaction_count(address).await.unwrap_or(0);

        Ok(AddressDisplay {
            address: address.to_string(),
            is_contract: false,
            balance_eth: balance_eth_val,
            usd_value: if eth_price > 0.0 {
                Some(format!("${:.2} USD", balance_usd_val))
            } else {
                None
            },
            transaction_count: tx_count,
            contract_name: None,
            contract_creator: None,
            creation_transaction: None,
            token_balances,
        })
    }
}
