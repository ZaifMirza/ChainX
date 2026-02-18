// Application configuration and state

use crate::api::RpcClient;
use crate::config::ChainConfig;
use crate::error::Result;

pub struct AppConfig {
    pub chain: ChainConfig,
    pub etherscan_api_key: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();

        let etherscan_api_key = std::env::var("ETHERSCAN_API_KEY").map_err(|_| {
            crate::error::ExplorerError::ConfigError(
                "ETHERSCAN_API_KEY not found in .env file".to_string(),
            )
        })?;

        let chain = crate::config::get_chain("ethereum");

        Ok(Self {
            chain,
            etherscan_api_key,
        })
    }
}

pub struct AppState {
    pub config: AppConfig,
    pub rpc_client: RpcClient,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let rpc_client = RpcClient::new(config.chain.rpc_url);

        Self { config, rpc_client }
    }
}
