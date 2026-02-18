// Block models

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockInfo {
    #[serde(rename = "number")]
    pub number: Option<String>,
    #[serde(rename = "hash")]
    pub hash: Option<String>,
    #[serde(rename = "parentHash")]
    pub parent_hash: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(rename = "miner")]
    pub miner: Option<String>,
    #[serde(rename = "difficulty")]
    pub difficulty: Option<String>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<String>,
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<String>,
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "uncles")]
    pub uncles: Option<Vec<serde_json::Value>>,
    #[serde(rename = "withdrawals")]
    pub withdrawals: Option<Vec<serde_json::Value>>,
}

pub trait BlockStats {
    fn transaction_count(&self) -> usize;
    fn uncle_count(&self) -> usize;
    fn withdrawal_count(&self) -> usize;
    fn gas_usage_percent(&self) -> f64;
}

impl BlockStats for BlockInfo {
    fn transaction_count(&self) -> usize {
        self.transactions.as_ref().map(|t| t.len()).unwrap_or(0)
    }

    fn uncle_count(&self) -> usize {
        self.uncles.as_ref().map(|u| u.len()).unwrap_or(0)
    }

    fn withdrawal_count(&self) -> usize {
        self.withdrawals.as_ref().map(|w| w.len()).unwrap_or(0)
    }

    fn gas_usage_percent(&self) -> f64 {
        let gas_used = crate::utils::hex::parse_hex(self.gas_used.as_deref().unwrap_or("0x0"));
        let gas_limit = crate::utils::hex::parse_hex(self.gas_limit.as_deref().unwrap_or("0x1"));
        if gas_limit == 0 {
            return 0.0;
        }
        (gas_used as f64 / gas_limit as f64) * 100.0
    }
}
