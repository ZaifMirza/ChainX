// Block command handler

use crate::app::AppState;
use crate::formatting::print_block_details;
use crate::error::Result;

pub struct BlockCommand;

impl BlockCommand {
    pub async fn execute(state: &AppState, block_number: u64) -> Result<()> {
        Self::print_fetching_message();
        
        match state.rpc_client.get_block(block_number, true).await {
            Ok(block) => {
                print_block_details(&block, state.config.chain.symbol);
                Ok(())
            }
            Err(e) => {
                println!("❌ Error: {}", e);
                Err(crate::error::ExplorerError::RpcError(e.to_string()))
            }
        }
    }
    
    fn print_fetching_message() {
        println!("\n📦 Fetching block details...\n");
    }
}
