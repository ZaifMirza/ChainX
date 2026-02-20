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
        // Try to load API key from config file
        let etherscan_api_key = crate::api_key::load_api_key()
            .map_err(|e| crate::error::ExplorerError::ConfigError(
                format!("Failed to load API key: {}", e)
            ))?;

        let chain = crate::config::get_chain("ethereum");

        Ok(Self {
            chain,
            etherscan_api_key,
        })
    }
    
    /// Check if API key is configured
    pub fn has_api_key(&self) -> bool {
        self.etherscan_api_key.is_some()
    }
    
    /// Get the API key or empty string
    pub fn get_api_key(&self) -> &str {
        self.etherscan_api_key.as_deref().unwrap_or("")
    }
    
    /// Update the API key in memory and persist to config file
    pub fn set_api_key(&mut self, api_key: String) -> Result<()> {
        crate::api_key::save_api_key(&api_key)
            .map_err(|e| crate::error::ExplorerError::ConfigError(
                format!("Failed to save API key: {}", e)
            ))?;
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
