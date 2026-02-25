// Address command handler

use crate::app::{AppState, Result, ExplorerError};
use crate::api::{EtherscanClient, get_eth_price};
use crate::models::{TokenInfo, AddressDisplay, TokenBalanceDisplay};
use crate::utils::wei_to_eth;
use std::collections::HashSet;

pub struct AddressCommand;

impl AddressCommand {
    /// Execute for TUI - returns AddressDisplay instead of printing
    pub async fn execute_tui(state: &AppState, address: &str) -> Result<AddressDisplay> {
        let api_key = state.config.etherscan_api_key.as_ref()
            .ok_or_else(api_key_not_configured_error)?;
        
        let eth_balance = fetch_eth_balance(&state.rpc_client, address).await?;
        let eth_price = fetch_eth_price(api_key).await;
        let transfers = fetch_token_transfers(api_key, address).await;
        
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        let token_balances = process_token_balances(&transfers);
        let tx_count = state.rpc_client.get_transaction_count(address).await.unwrap_or(0);

        Ok(AddressDisplay {
            address: address.to_string(),
            is_contract: false,
            balance_eth: balance_eth_val,
            usd_value: format_usd_value(eth_price, balance_usd_val),
            transaction_count: tx_count,
            contract_name: None,
            contract_creator: None,
            creation_transaction: None,
            token_balances,
        })
    }
}

fn api_key_not_configured_error() -> ExplorerError {
    ExplorerError::ConfigError(
        "Etherscan API key not configured. Press 's' to set up your API key.".to_string()
    )
}

async fn fetch_eth_balance(client: &crate::api::RpcClient, address: &str) -> Result<String> {
    client.get_balance(address).await
        .map_err(|e| ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))
}

async fn fetch_eth_price(api_key: &str) -> f64 {
    get_eth_price(api_key).await
        .map(|(price, _)| price)
        .unwrap_or(0.0)
}

async fn fetch_token_transfers(api_key: &str, address: &str) -> Vec<crate::models::TokenTransfer> {
    EtherscanClient::new(api_key.to_string())
        .get_token_transactions(address)
        .await
        .unwrap_or_default()
}

fn process_token_balances(transfers: &[crate::models::TokenTransfer]) -> Vec<TokenBalanceDisplay> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut token_balances: Vec<TokenBalanceDisplay> = Vec::new();
    
    for transfer in transfers {
        if transfer.is_suspicious() {
            continue;
        }
        
        if seen.insert(transfer.contract_address.clone()) {
            token_balances.push(TokenBalanceDisplay::from_transfer(transfer));
        }
        
        if token_balances.len() >= 10 {
            break;
        }
    }
    
    token_balances
}

fn format_usd_value(eth_price: f64, balance_usd: f64) -> Option<String> {
    if eth_price > 0.0 {
        Some(format!("${:.2} USD", balance_usd))
    } else {
        None
    }
}
