// Transaction models

use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<String>,
}

#[allow(dead_code)]
pub trait TransactionFeeCalculator {
    fn calculate_fee(&self, receipt: &TransactionReceipt) -> f64;
}

#[allow(dead_code)]
impl TransactionFeeCalculator for TransactionDetail {
    fn calculate_fee(&self, receipt: &TransactionReceipt) -> f64 {
        let gas_used = receipt
            .gas_used
            .as_deref()
            .map(|g| crate::utils::hex::parse_hex(g))
            .unwrap_or(0);

        let gas_price = crate::utils::hex::parse_hex(&self.gas_price);
        (gas_used as u128 * gas_price as u128) as f64 / 1e18
    }
}
