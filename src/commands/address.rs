// Address command handler with rate limiting

use crate::app::AppState;
use crate::api::{EtherscanClient, get_eth_price};
use crate::models::TokenInfo;
use crate::utils::wei_to_eth;
use crate::error::Result;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::sleep;

pub struct AddressCommand;

impl AddressCommand {
    pub async fn execute(state: &AppState, address: &str) -> Result<()> {
        Self::print_fetching_message();
        
        // Fetch ETH balance first (no rate limit)
        let eth_balance = state.rpc_client.get_balance(address).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch balance: {}", e)))?;
        
        // Wait before Etherscan API call
        sleep(Duration::from_millis(350)).await;
        
        // Fetch ETH price
        let (eth_price, _) = get_eth_price(&state.config.etherscan_api_key).await
            .map_err(|e| crate::error::ExplorerError::ApiError(format!("Failed to fetch ETH price: {}", e)))?;
        
        // Wait before next Etherscan API call
        sleep(Duration::from_millis(350)).await;
        
        // Fetch token transactions
        let etherscan = EtherscanClient::new(state.config.etherscan_api_key.clone());
        let transfers = match etherscan.get_token_transactions(address).await {
            Ok(t) => t,
            Err(e) => {
                println!("⚠️  Token fetch failed: {}\n", e);
                vec![]
            }
        };
        
        // Calculate values
        let balance_eth_val = wei_to_eth(&eth_balance);
        let balance_usd_val = balance_eth_val * eth_price;
        
        // Print results
        Self::print_address_details(address, balance_eth_val, balance_usd_val);
        Self::print_token_transfers(&transfers);
        
        Ok(())
    }
    
    fn print_fetching_message() {
        println!("\n📋 Fetching address details from Etherscan...\n");
    }
    
    fn print_address_details(address: &str, balance_eth: f64, balance_usd: f64) {
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║                  ADDRESS DETAILS                          ║");
        println!("╠═══════════════════════════════════════════════════════════╣");
        println!("║  {:18} │ {}  ║", "Address:", crate::utils::text::truncate_and_pad(address, 28));
        println!("║  {:18} │ {:.8} ETH                   ║", "ETH Balance:", balance_eth);
        println!("║  {:18} │ ${:.2} USD                      ║", "USD Value:", balance_usd);
    }
    
    fn print_token_transfers(transfers: &[crate::models::TokenTransfer]) {
        if transfers.is_empty() {
            println!("╚═══════════════════════════════════════════════════════════╝");
            return;
        }
        
        let mut seen: HashSet<String> = HashSet::new();
        let mut unique_tokens: Vec<&crate::models::TokenTransfer> = Vec::new();
        let mut filtered_count = 0;
        
        for t in transfers {
            if t.is_suspicious() {
                filtered_count += 1;
                continue;
            }
            if !seen.contains(&t.contract_address) {
                seen.insert(t.contract_address.clone());
                unique_tokens.push(t);
            }
            if unique_tokens.len() >= crate::config::constants::MAX_TOKEN_DISPLAY {
                break;
            }
        }
        
        if !unique_tokens.is_empty() {
            Self::print_token_list(&unique_tokens, transfers.len(), filtered_count);
        } else if filtered_count > 0 {
            println!("╠═══════════════════════════════════════════════════════════╣");
            println!("║  ⚠️  All transfers filtered (scam/spam tokens detected)    ║");
        }
        
        println!("╚═══════════════════════════════════════════════════════════╝");
    }
    
    fn print_token_list(
        tokens: &[&crate::models::TokenTransfer],
        total_count: usize,
        filtered_count: usize,
    ) {
        println!("╠═══════════════════════════════════════════════════════════╣");
        println!("║  RECENT TOKEN TRANSFERS (Filtered)                        ║");
        println!("╠═══════════════════════════════════════════════════════════╣");
        
        for token in tokens {
            let value = token.display_value();
            let symbol = token.display_symbol();
            let symbol_display = crate::utils::text::truncate_for_display(&symbol, 10);
            let value_display = if value > 1_000_000.0 {
                format!("{:.2e}", value)
            } else {
                format!("{:.8}", value)
            };
            println!("║  {:>18} │ {:>20} ║", symbol_display, value_display);
        }
        
        let remaining = total_count - tokens.len() - filtered_count;
        if remaining > 0 || filtered_count > 0 {
            println!("║  {:>18} │ {:>20} ║",
                if filtered_count > 0 { "⚠️ Scams filtered" } else { "" },
                format!("... {} more", remaining)
            );
        }
    }
}
