// Price data models

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EthPriceData {
    pub ethbtc: String,
    pub ethusd: String,
}

pub trait PriceExtractor {
    fn price_usd(&self) -> Result<f64, String>;
    fn price_btc(&self) -> Result<f64, String>;
}

impl PriceExtractor for EthPriceData {
    fn price_usd(&self) -> Result<f64, String> {
        self.ethusd
            .parse()
            .map_err(|e| format!("Failed to parse ethusd: {}", e))
    }

    fn price_btc(&self) -> Result<f64, String> {
        self.ethbtc
            .parse()
            .map_err(|e| format!("Failed to parse ethbtc: {}", e))
    }
}
