// Application configuration and state

use crate::api::RpcClient;
use crate::config::ChainConfig;
use crate::error::Result;

pub struct AppConfig {
    pub chain: ChainConfig,
    pub etherscan_api_key: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let etherscan_api_key = load_api_key_safe()?;
        let chain = crate::config::get_chain("ethereum");

        Ok(Self {
            chain,
            etherscan_api_key,
        })
    }
    
    pub fn has_api_key(&self) -> bool {
        self.etherscan_api_key.is_some()
    }
    
    pub fn set_api_key(&mut self, api_key: String) -> Result<()> {
        save_api_key_safe(&api_key)?;
        self.etherscan_api_key = Some(api_key);
        Ok(())
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

fn load_api_key_safe() -> Result<Option<String>> {
    crate::api_key::load_api_key()
        .map_err(|e| crate::error::ExplorerError::ConfigError(
            format!("Failed to load API key: {}", e)
        ))
}

fn save_api_key_safe(api_key: &str) -> Result<()> {
    crate::api_key::save_api_key(api_key)
        .map_err(|e| crate::error::ExplorerError::ConfigError(
            format!("Failed to save API key: {}", e)
        ))
}
