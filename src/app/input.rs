// Input reading and parsing

use super::error::{ExplorerError, Result};
use crate::validation::input::{AddressValidator, BlockNumberValidator, InputValidator};

#[derive(Debug, Clone)]
pub enum InputType {
    Address(String),
    BlockNumber(u64),
    TransactionHash(String),
}

pub struct InputParser;

impl InputParser {
    pub fn parse(input: &str) -> Result<InputType> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(ExplorerError::ValidationError(
                "Input cannot be empty".to_string(),
            ));
        }

        // Check for address
        if AddressValidator(trimmed).is_valid() {
            return Ok(InputType::Address(trimmed.to_string()));
        }

        // Check for block number
        if BlockNumberValidator(trimmed).is_valid() {
            let block_number = if let Some(stripped) = trimmed.strip_prefix("0x") {
                u64::from_str_radix(stripped, 16).unwrap_or(0)
            } else {
                trimmed.parse().unwrap_or(0)
            };
            return Ok(InputType::BlockNumber(block_number));
        }

        // Assume it's a transaction hash
        Ok(InputType::TransactionHash(trimmed.to_string()))
    }
}
