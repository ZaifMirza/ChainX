// Etherscan URL builder

pub struct EtherscanUrlBuilder {
    base_url: String,
    chain_id: u32,
    api_key: String,
}

impl EtherscanUrlBuilder {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://api.etherscan.io/v2/api".to_string(),
            chain_id: 1,
            api_key: api_key.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn chain_id(mut self, id: u32) -> Self {
        self.chain_id = id;
        self
    }

    fn build_base(&self) -> String {
        format!(
            "{}?chainid={}&apikey={}",
            self.base_url, self.chain_id, self.api_key
        )
    }

    pub fn token_transactions(&self, address: &str) -> String {
        format!(
            "{}&module=account&action=tokentx&address={}&page=1&offset=100&sort=desc",
            self.build_base(),
            address
        )
    }

    pub fn eth_price(&self) -> String {
        format!("{}&module=stats&action=ethprice", self.build_base())
    }
}
