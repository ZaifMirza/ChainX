// Address model

use crate::models::token::TokenTransfer;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AddressInfo {
    pub address: String,
    pub balance: String,
    #[serde(rename = "transactionCount")]
    pub transaction_count: Option<String>,
    #[serde(rename = "contractCode")]
    pub contract_code: Option<String>,
}

impl AddressInfo {
    pub fn balance_eth(&self) -> f64 {
        let wei = crate::utils::hex::parse_hex(&self.balance);
        wei as f64 / 1e18
    }

    pub fn is_contract(&self) -> bool {
        self.contract_code
            .as_ref()
            .map(|code| !code.is_empty() && code != "0x")
            .unwrap_or(false)
    }

    pub fn tx_count(&self) -> u64 {
        self.transaction_count
            .as_deref()
            .map(|c| crate::utils::hex::parse_hex(c))
            .unwrap_or(0)
    }
}

// Display-ready address for TUI
#[derive(Debug, Clone)]
pub struct AddressDisplay {
    pub address: String,
    pub is_contract: bool,
    pub balance_eth: f64,
    pub usd_value: Option<String>,
    pub transaction_count: u64,
    pub contract_name: Option<String>,
    pub contract_creator: Option<String>,
    pub creation_transaction: Option<String>,
    pub token_balances: Vec<TokenBalanceDisplay>,
}

impl AddressDisplay {
    pub fn new(address: String) -> Self {
        Self {
            address,
            is_contract: false,
            balance_eth: 0.0,
            usd_value: None,
            transaction_count: 0,
            contract_name: None,
            contract_creator: None,
            creation_transaction: None,
            token_balances: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenBalanceDisplay {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub balance_formatted: String,
    pub decimals: u32,
    pub contract_address: String,
}

impl TokenBalanceDisplay {
    pub fn from_transfer(transfer: &TokenTransfer) -> Self {
        let decimals: u32 = transfer.token_decimal.parse().unwrap_or(18);
        let balance = crate::utils::units::parse_token_balance(&transfer.value, decimals);

        Self {
            symbol: Some(transfer.token_symbol.clone()),
            name: Some(transfer.token_name.clone()),
            balance_formatted: format!("{:.4}", balance),
            decimals,
            contract_address: transfer.contract_address.clone(),
        }
    }
}
