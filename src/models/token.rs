// Token models

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TokenTransfer {
    #[allow(dead_code)]
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[allow(dead_code)]
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[allow(dead_code)]
    #[serde(rename = "hash")]
    pub hash: String,
    #[allow(dead_code)]
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[allow(dead_code)]
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "tokenDecimal")]
    pub token_decimal: String,
}

pub trait TokenInfo {
    fn is_suspicious(&self) -> bool;
    fn display_symbol(&self) -> String;
    fn display_value(&self) -> f64;
}

impl TokenInfo for TokenTransfer {
    fn is_suspicious(&self) -> bool {
        let combined = format!("{} {}", self.token_symbol, self.token_name).to_lowercase();

        if combined.contains("http")
            || combined.contains("www.")
            || combined.contains(".org")
            || combined.contains(".com")
            || combined.contains(".net")
            || combined.contains("visit website")
            || combined.contains("claim")
            || combined.contains("reward")
            || combined.contains("airdrop")
        {
            return true;
        }

        if self.token_symbol.len() > 20 || self.token_name.len() > 50 {
            return true;
        }

        crate::validation::unicode::contains_spoofing(&self.token_symbol)
            || crate::validation::unicode::contains_spoofing(&self.token_name)
    }

    fn display_symbol(&self) -> String {
        if crate::validation::unicode::contains_spoofing(&self.token_symbol) {
            "⚠️ SPOOFED".to_string()
        } else {
            self.token_symbol.clone()
        }
    }

    fn display_value(&self) -> f64 {
        let decimals: u32 = self.token_decimal.parse().unwrap_or(18);
        crate::utils::units::parse_token_balance(&self.value, decimals)
    }
}
