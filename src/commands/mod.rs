// Command router

pub mod address;
pub mod block;
pub mod contract;
pub mod transaction;

use crate::app::{AppState, input::InputType};
use crate::validation::ContractDetector;
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
}
