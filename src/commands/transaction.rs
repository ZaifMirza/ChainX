// Transaction command handler

use crate::app::AppState;
use crate::error::Result;
use crate::models::{TransactionDisplay, TransactionReceipt};

pub struct TransactionCommand;

impl TransactionCommand {
    /// Execute for TUI - returns TransactionDisplay instead of printing
    pub async fn execute_tui(state: &AppState, tx_hash: &str) -> Result<TransactionDisplay> {
        let (tx_detail, receipt, _timestamp) = state.rpc_client.fetch_transaction_data(tx_hash).await
            .map_err(|e| crate::error::ExplorerError::RpcError(e.to_string()))?;

        let current_block = state.rpc_client.get_block_number().await
            .map_err(|e| crate::error::ExplorerError::RpcError(e.to_string()))?;

        let receipt = receipt.unwrap_or_else(|| TransactionReceipt {
            status: Some("0x0".to_string()),
            gas_used: Some("0x0".to_string()),
            transaction_hash: Some(tx_hash.to_string()),
            block_number: tx_detail.block_number.clone(),
        });

        Ok(TransactionDisplay::from_detail(
            &tx_detail,
            &receipt,
            current_block,
        ))
    }
}
