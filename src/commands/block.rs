// Block command handler

use crate::app::{AppState, Result, ExplorerError};
use crate::models::{BlockInfo, BlockDisplay};

pub struct BlockCommand;

impl BlockCommand {
    /// Execute for TUI - returns BlockDisplay instead of printing
    pub async fn execute_tui(state: &AppState, block_number: u64) -> Result<BlockDisplay> {
        let block: BlockInfo = state.rpc_client.get_block(block_number, true).await
            .map_err(|e| ExplorerError::RpcError(e.to_string()))?;

        let current_block = state.rpc_client.get_block_number().await
            .map_err(|e| ExplorerError::RpcError(e.to_string()))?;

        Ok(BlockDisplay::from_block_info(&block, current_block))
    }
}
