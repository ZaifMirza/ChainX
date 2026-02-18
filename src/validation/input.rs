// Input validation module

use crate::config::constants::{ETH_ADDRESS_LENGTH, TX_HASH_LENGTH};

pub trait InputValidator {
    fn is_valid(&self) -> bool;
}

pub struct AddressValidator<'a>(pub &'a str);

impl<'a> InputValidator for AddressValidator<'a> {
    fn is_valid(&self) -> bool {
        let input = self.0;
        input.starts_with("0x")
            && input.len() == ETH_ADDRESS_LENGTH
            && input.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    }
}

pub struct BlockNumberValidator<'a>(pub &'a str);

impl<'a> InputValidator for BlockNumberValidator<'a> {
    fn is_valid(&self) -> bool {
        let input = self.0;
        if input.starts_with("0x") {
            if input.len() == TX_HASH_LENGTH {
                return false;
            }
            return true;
        }
        input.chars().all(|c| c.is_ascii_digit())
    }
}

#[allow(dead_code)]
pub struct TransactionHashValidator<'a>(pub &'a str);

impl<'a> InputValidator for TransactionHashValidator<'a> {
    fn is_valid(&self) -> bool {
        let input = self.0;
        input.starts_with("0x")
            && input.len() == TX_HASH_LENGTH
            && input.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    }
}


#[allow(dead_code)]
pub fn is_transaction_hash(input: &str) -> bool {
    TransactionHashValidator(input).is_valid()
}
