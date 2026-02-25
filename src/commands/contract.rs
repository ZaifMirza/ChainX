// Contract command handler

use crate::app::{AppState, Result, ExplorerError};
use crate::api::{get_eth_price, etherscan::ContractClient};
use crate::utils::wei_to_eth;
use crate::models::ContractDisplay;

pub struct ContractCommand;

impl ContractCommand {
    /// Execute for TUI - returns ContractDisplay instead of printing
    pub async fn execute_tui(state: &AppState, address: &str) -> Result<ContractDisplay> {
        let api_key = state.config.etherscan_api_key.as_ref()
            .ok_or_else(api_key_not_configured_error)?;
        
        let (eth_balance, eth_price) = fetch_balance_and_price(state, address, api_key).await
            .unwrap_or(("0x0".to_string(), 0.0));
        
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        let contract_client = ContractClient::new(api_key.clone());
        let contract_info = contract_client.get_contract_info(address).await.ok();
        let creation_info = contract_client.get_contract_creation(address).await.ok();
        let transactions = contract_client.get_contract_transactions(address, 5).await.unwrap_or_default();
        
        let mut display = ContractDisplay::new(address.to_string());
        display.balance_eth = balance_eth_val;
        display.usd_value = format_usd_value(eth_price, balance_usd_val);
        display.transaction_count = transactions.len();
        display.recent_transactions = transactions;

        if let Some(info) = contract_info {
            populate_contract_info(&mut display, &info);
        }

        if let Some(creation) = creation_info {
            display.creator = Some(creation.contract_creator);
            display.creation_transaction = Some(creation.tx_hash);
        }
        
        Ok(display)
    }
}

fn api_key_not_configured_error() -> ExplorerError {
    ExplorerError::ConfigError(
        "Etherscan API key not configured. Press 's' to set up your API key.".to_string()
    )
}

async fn fetch_balance_and_price(
    state: &AppState, 
    address: &str, 
    api_key: &str
) -> Result<(String, f64)> {
    let eth_balance = state.rpc_client.get_balance(address).await
        .map_err(|e| ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))?;
    
    let (eth_price, _) = get_eth_price(api_key).await
        .map_err(|e| ExplorerError::ApiError(format!("Failed to fetch ETH price: {}", e)))?;
    
    Ok((eth_balance, eth_price))
}

fn format_usd_value(eth_price: f64, balance_usd: f64) -> Option<String> {
    if eth_price > 0.0 {
        Some(format!("${:.2} USD", balance_usd))
    } else {
        None
    }
}

fn populate_contract_info(display: &mut ContractDisplay, info: &crate::models::ContractInfo) {
    if !info.contract_name.is_empty() {
        display.name = Some(info.contract_name.clone());
    }
    if !info.compiler_version.is_empty() {
        display.compiler = Some(info.compiler_version.clone());
    }
    display.is_proxy = info.proxy == "1";
    if !info.implementation.is_empty() {
        display.implementation = Some(info.implementation.clone());
    }
}
