// Transaction models

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TransactionDetail {
    #[allow(dead_code)]
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "input", default)]
    pub input: String,
    #[serde(rename = "nonce")]
    pub nonce: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "gas")]
    pub gas: Option<String>,
}

impl TransactionDetail {
    pub fn block_number_u64(&self) -> u64 {
        self.block_number
            .as_deref()
            .map(|b| crate::utils::hex::parse_hex(b))
            .unwrap_or(0)
    }

    pub fn value_eth(&self) -> f64 {
        let wei = crate::utils::hex::parse_hex(&self.value);
        wei as f64 / 1e18
    }

    pub fn gas_price_gwei(&self) -> f64 {
        let wei = crate::utils::hex::parse_hex(&self.gas_price);
        wei as f64 / 1e9
    }

    pub fn nonce_u64(&self) -> u64 {
        self.nonce
            .as_deref()
            .map(|n| crate::utils::hex::parse_hex(n))
            .unwrap_or(0)
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
}

impl TransactionReceipt {
    pub fn is_success(&self) -> bool {
        self.status
            .as_deref()
            .map(|s| crate::utils::hex::parse_hex(s) == 1)
            .unwrap_or(false)
    }

    pub fn gas_used_u64(&self) -> u64 {
        self.gas_used
            .as_deref()
            .map(|g| crate::utils::hex::parse_hex(g))
            .unwrap_or(0)
    }
}

pub trait TransactionFeeCalculator {
    fn calculate_fee(&self, receipt: &TransactionReceipt) -> f64;
}

impl TransactionFeeCalculator for TransactionDetail {
    fn calculate_fee(&self, receipt: &TransactionReceipt) -> f64 {
        let gas_used = receipt.gas_used_u64();
        let gas_price = crate::utils::hex::parse_hex(&self.gas_price);
        (gas_used as u128 * gas_price as u128) as f64 / 1e18
    }
}

// Display-ready transaction for TUI
#[derive(Debug, Clone)]
pub struct TransactionDisplay {
    pub hash: String,
    pub status: bool,
    pub block_number: u64,
    pub confirmations: u64,
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub value_eth: f64,
    pub gas_used: String,
    pub gas_price_gwei: f64,
    pub transaction_fee_eth: f64,
    pub nonce: u64,
    pub input_data: Option<String>,
}

impl TransactionDisplay {
    pub fn from_detail(
        detail: &TransactionDetail,
        receipt: &TransactionReceipt,
        block_number: u64,
    ) -> Self {
        Self {
            hash: detail.hash.clone(),
            status: receipt.is_success(),
            block_number: detail.block_number_u64(),
            confirmations: block_number.saturating_sub(detail.block_number_u64()),
            timestamp: "Unknown".to_string(), // Would need block timestamp
            from: detail.from.clone(),
            to: detail.to.clone(),
            value_eth: detail.value_eth(),
            gas_used: format!("{}", receipt.gas_used_u64()),
            gas_price_gwei: detail.gas_price_gwei(),
            transaction_fee_eth: detail.calculate_fee(receipt),
            nonce: detail.nonce_u64(),
            input_data: if detail.input.len() > 2 {
                Some(detail.input.clone())
            } else {
                None
            },
        }
    }
}
