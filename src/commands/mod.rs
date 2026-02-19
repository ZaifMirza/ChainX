// Command router

pub mod address;
pub mod block;
pub mod contract;
pub mod transaction;

use crate::app::{AppState, input::InputType};
use crate::validation::ContractDetector;
use crate::tui::app::ViewState;

use address::AddressCommand;
use block::BlockCommand;
use contract::ContractCommand;
use transaction::TransactionCommand;
use crate::error::Result;

pub struct CommandRouter;

impl CommandRouter {
    pub async fn route(state: &AppState, input_type: InputType) -> Result<()> {
        match input_type {
            InputType::Address(addr) => {
                // Detect if it's a contract or regular address
                let detector = ContractDetector::new(&state.rpc_client, &addr);
                let is_contract = detector.is_contract().await;
                
                if is_contract {
                    ContractCommand::execute(state, &addr).await
                } else {
                    AddressCommand::execute(state, &addr).await
                }
            }
            InputType::BlockNumber(num) => {
                BlockCommand::execute(state, num).await
            }
            InputType::TransactionHash(hash) => {
                TransactionCommand::execute(state, &hash).await
            }
        }
    }

    /// Route for TUI - returns ViewState instead of printing
    pub async fn route_tui(state: &AppState, input_type: InputType) -> Result<ViewState> {
        match input_type {
            InputType::Address(addr) => {
                // Detect if it's a contract or regular address
                let detector = ContractDetector::new(&state.rpc_client, &addr);
                let is_contract = detector.is_contract().await;
                
                if is_contract {
                    let contract = ContractCommand::execute_tui(state, &addr).await?;
                    Ok(ViewState::Contract(Box::new(contract)))
                } else {
                    let address = AddressCommand::execute_tui(state, &addr).await?;
                    Ok(ViewState::Address(Box::new(address)))
                }
            }
            InputType::BlockNumber(num) => {
                let block = BlockCommand::execute_tui(state, num).await?;
                Ok(ViewState::Block(Box::new(block)))
            }
            InputType::TransactionHash(hash) => {
                let tx = TransactionCommand::execute_tui(state, &hash).await?;
                Ok(ViewState::Transaction(Box::new(tx)))
            }
        }
    }
}
