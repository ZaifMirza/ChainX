// Input reading and parsing

use crate::error::{ExplorerError, Result};
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
            let block_number = if trimmed.starts_with("0x") {
                u64::from_str_radix(&trimmed[2..], 16).unwrap_or(0)
            } else {
                trimmed.parse().unwrap_or(0)
            };
            return Ok(InputType::BlockNumber(block_number));
        }

        // Assume it's a transaction hash
        Ok(InputType::TransactionHash(trimmed.to_string()))
    }
}

pub fn read_user_input() -> Result<String> {
    println!("Enter transaction hash (0x...), block number, contract address (0x...), or 'quit'/'exit' to exit:");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| ExplorerError::ValidationError(format!("Failed to read input: {}", e)))?;

    Ok(input)
}
