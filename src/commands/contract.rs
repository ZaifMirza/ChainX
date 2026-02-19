// Contract command handler with rate limiting

use crate::app::AppState;
use crate::api::{get_eth_price, etherscan::ContractClient};
use crate::utils::wei_to_eth;
use crate::error::Result;
use crate::models::{ContractDisplay, ContractInfo, ContractCreationInfo, ContractTransaction};
use std::time::Duration;
use tokio::time::sleep;

pub struct ContractCommand;

impl ContractCommand {
    pub async fn execute(state: &AppState, address: &str) -> Result<()> {
        Self::print_fetching_message();
        
        // Fetch ETH balance and price first (these don't count against Etherscan rate limit as heavily)
        let (eth_balance, eth_price) = Self::fetch_balance_and_price(state, address).await?;
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        // Wait before making Etherscan API calls (rate limit: 3 calls/sec)
        sleep(Duration::from_millis(350)).await;
        
        // Create contract client
        let contract_client = ContractClient::new(state.config.etherscan_api_key.clone());
        
        // Fetch contract info with retry
        let contract_info = Self::fetch_with_retry(
            || async { contract_client.get_contract_info(address).await },
            3
        ).await
        .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch contract info: {}", e)))?;
        
        // Wait between API calls
        sleep(Duration::from_millis(350)).await;
        
        // Fetch creation info with retry
        let creation_info = Self::fetch_with_retry(
            || async { contract_client.get_contract_creation(address).await },
            3
        ).await
        .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch creation info: {}", e)))?;
        
        // Wait between API calls
        sleep(Duration::from_millis(350)).await;
        
        // Fetch transactions with retry
        let transactions = Self::fetch_with_retry(
            || async { contract_client.get_contract_transactions(address, 5).await },
            3
        ).await
        .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch transactions: {}", e)))?;
        
        // Print results
        Self::print_contract_details(address, balance_eth_val, balance_usd_val, &contract_info, &creation_info);
        Self::print_recent_transactions(&transactions);
        
        Ok(())
    }

    /// Execute for TUI - returns ContractDisplay instead of printing
    pub async fn execute_tui(state: &AppState, address: &str) -> Result<ContractDisplay> {
        // Fetch ETH balance and price
        let (eth_balance, eth_price) = Self::fetch_balance_and_price(state, address).await
            .unwrap_or(("0x0".to_string(), 0.0));
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        // Create contract client
        let contract_client = ContractClient::new(state.config.etherscan_api_key.clone());
        
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
    
    async fn fetch_balance_and_price(state: &AppState, address: &str) -> Result<(String, f64)> {
        let eth_balance = state.rpc_client.get_balance(address).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))?;
        
        let (eth_price, _) = get_eth_price(&state.config.etherscan_api_key).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch ETH price: {}", e)))?;
        
        Ok((eth_balance, eth_price))
    }
    
    async fn fetch_with_retry<F, Fut, T>(f: F, max_retries: u32) -> crate::api::error::ApiResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = crate::api::error::ApiResult<T>>,
    {
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let error_msg = e.to_string();
                    // Check if it's a rate limit error
                    if error_msg.contains("rate limit") || error_msg.contains("Max calls") {
                        let delay = Duration::from_millis(500 * (attempt + 1) as u64);
                        eprintln!("⚠️  Rate limit hit, waiting {}ms before retry... (attempt {}/{})", 
                                 delay.as_millis(), attempt + 1, max_retries);
                        sleep(delay).await;
                        last_error = Some(e);
                    } else {
                        // Not a rate limit error, return immediately
                        return Err(e);
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| 
            crate::api::error::ApiError::RpcError("Max retries exceeded".to_string())
        ))
    }
    
    fn print_fetching_message() {
        println!("\n📋 Fetching contract details from Etherscan...\n");
    }
    
    fn print_contract_details(
        address: &str,
        balance_eth: f64,
        balance_usd: f64,
        info: &ContractInfo,
        creation: &ContractCreationInfo,
    ) {
        println!("╔════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                              CONTRACT DETAILS                                      ║");
        println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║  {:18} │ {:66} ║", "Contract:", address);
        println!("║  {:18} │ {:.8} ETH                                                        ║", "ETH Balance:", balance_eth);
        println!("║  {:18} │ ${:.2} USD                                                          ║", "USD Value:", balance_usd);
        println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║  CONTRACT INFORMATION                                                              ║");
        println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
        
        // Contract name (handle unverified)
        let name = if info.contract_name.is_empty() {
            "Unverified Contract"
        } else {
            &info.contract_name
        };
        println!("║  {:18} │ {:66} ║", "Name:", name);
        
        println!("║  {:18} │ {:66} ║", "Creator:", &creation.contract_creator);
        println!("║  {:18} │ {:66} ║", "Compiler:", &info.compiler_version);
        println!("║  {:18} │ {:66} ║", "Creation Tx:", &creation.tx_hash);
        
        if !info.contract_name.is_empty() {
            if info.proxy == "1" {
                println!("║  {:18} │ {:66} ║", "Type:", "Proxy Contract");
                println!("║  {:18} │ {:66} ║", "Implementation:", &info.implementation);
            }
        }
    }
    
    fn print_recent_transactions(transactions: &[ContractTransaction]) {
        if transactions.is_empty() {
            println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
            println!("║  No recent transactions found                                                      ║");
            println!("╚════════════════════════════════════════════════════════════════════════════════════╝");
            return;
        }
        
        println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║  LAST 5 TRANSACTIONS                                                               ║");
        println!("╠════════════════════════════════════════════════════════════════════════════════════╣");
        
        for (i, tx) in transactions.iter().enumerate().take(5) {
            let value_eth = crate::utils::wei_to_eth(&tx.value);
            let status_icon = if tx.status == "1" { "✅" } else { "❌" };
            
            println!("║  Transaction #{:<3}                                                                   ║", i + 1);
            println!("║  {:18} │ {:66} ║", "Hash:", &tx.hash);
            println!("║  {:18} │ {:.6} ETH                                                        ║", "Value:", value_eth);
            println!("║  {:18} │ {:66} ║", "Status:", status_icon);
            
            if !tx.function_name.is_empty() {
                println!("║  {:18} │ {:66} ║", "Function:", &tx.function_name);
            }
            
            if i < transactions.len().min(5) - 1 {
                println!("║                                                                                    ║");
            }
        }
        
        println!("╚════════════════════════════════════════════════════════════════════════════════════╝");
    }
}