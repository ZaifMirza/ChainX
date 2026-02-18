// Transaction command handler

use crate::app::AppState;
use crate::formatting::print_transaction_details;
use crate::error::Result;

pub struct TransactionCommand;

impl TransactionCommand {
    pub async fn execute(state: &AppState, tx_hash: &str) -> Result<()> {
        Self::print_fetching_message();
        
        match state.rpc_client.fetch_transaction_data(tx_hash).await {
            Ok((tx_detail, receipt, timestamp)) => {
                print_transaction_details(
                    &tx_detail,
                    &receipt,
                    timestamp.as_deref(),
                    state.config.chain.symbol,
                );
                Ok(())
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                Err(crate::error::ExplorerError::RpcError(e.to_string()))
            }
        }
    }
    
    fn print_fetching_message() {
        println!("\n🔍 Fetching transaction details...\n");
    }
}
