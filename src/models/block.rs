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
    #[serde(rename = "stateRoot")]
    pub state_root: Option<String>,
    #[serde(rename = "extraData")]
    pub extra_data: Option<String>,
    #[serde(rename = "baseFeePerGas")]
    #[allow(dead_code)]
    pub base_fee_per_gas: Option<String>,
}

impl BlockInfo {
    pub fn number_u64(&self) -> u64 {
        self.number
            .as_deref()
            .map(|n| crate::utils::hex::parse_hex(n))
            .unwrap_or(0)
    }

    pub fn timestamp_u64(&self) -> u64 {
        self.timestamp
            .as_deref()
            .map(|t| crate::utils::hex::parse_hex(t))
            .unwrap_or(0)
    }

    pub fn gas_used_u64(&self) -> u64 {
        self.gas_used
            .as_deref()
            .map(|g| crate::utils::hex::parse_hex(g))
            .unwrap_or(0)
    }

    pub fn gas_limit_u64(&self) -> u64 {
        self.gas_limit
            .as_deref()
            .map(|g| crate::utils::hex::parse_hex(g))
            .unwrap_or(0)
    }
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
        let gas_used = self.gas_used_u64();
        let gas_limit = self.gas_limit_u64();
        if gas_limit == 0 {
            return 0.0;
        }
        (gas_used as f64 / gas_limit as f64) * 100.0
    }
}

// Display-ready block for TUI
#[derive(Debug, Clone)]
pub struct BlockDisplay {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub confirmed: bool,
    pub timestamp: String,
    pub age: String,
    pub transactions: usize,
    pub withdrawals: Option<usize>,
    pub block_reward: Option<String>,
    pub gas_used: String,
    pub gas_percentage: Option<String>,
    pub gas_limit: String,
    pub miner: String,
    pub state_root: Option<String>,
    pub extra_data: Option<String>,
}

impl BlockDisplay {
    pub fn from_block_info(block: &BlockInfo, current_block: u64) -> Self {
        let gas_percent = block.gas_usage_percent();
        let timestamp = block.timestamp_u64();

        Self {
            number: block.number_u64(),
            hash: block.hash.clone().unwrap_or_default(),
            parent_hash: block.parent_hash.clone().unwrap_or_default(),
            confirmed: current_block > block.number_u64(),
            timestamp: crate::utils::format_timestamp_u64(timestamp),
            age: crate::utils::format_age(timestamp),
            transactions: block.transaction_count(),
            withdrawals: block.withdrawals.as_ref().map(|w| w.len()),
            block_reward: Some(crate::utils::calculate_block_reward_str(block.number_u64())),
            gas_used: format!("{}", block.gas_used_u64()),
            gas_percentage: Some(format!("{:.1}%", gas_percent)),
            gas_limit: format!("{}", block.gas_limit_u64()),
            miner: block.miner.clone().unwrap_or_default(),
            state_root: block.state_root.clone(),
            extra_data: block.extra_data.as_ref().map(|e| {
                if e.starts_with("0x") {
                    hex::decode(&e[2..])
                        .ok()
                        .and_then(|bytes| String::from_utf8(bytes).ok())
                        .unwrap_or_else(|| e.clone())
                } else {
                    e.clone()
                }
            }),
        }
    }
}
