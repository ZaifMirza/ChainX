// Block command handler

use crate::app::AppState;
use crate::formatting::print_block_details;
use crate::error::Result;
use crate::models::{BlockInfo, BlockDisplay};

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

    /// Execute for TUI - returns BlockDisplay instead of printing
    pub async fn execute_tui(state: &AppState, block_number: u64) -> Result<BlockDisplay> {
        let block: BlockInfo = state.rpc_client.get_block(block_number, true).await
            .map_err(|e| crate::error::ExplorerError::RpcError(e.to_string()))?;

        // Get current block number for confirmation status
        let current_block = state.rpc_client.get_block_number().await
            .map_err(|e| crate::error::ExplorerError::RpcError(e.to_string()))?;

        Ok(BlockDisplay::from_block_info(&block, current_block))
    }
    
    fn print_fetching_message() {
        println!("\n📦 Fetching block details...\n");
    }
}
