//! Error types for the blockchain explorer

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("RPC Error: {0}")]
    RpcError(String),
    #[error("API Error: {0}")]
    ApiError(String),
    #[allow(dead_code)]
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Config Error: {0}")]
    ConfigError(String),
    #[allow(dead_code)] 
    #[error("Cache Error: {0}")]
    CacheError(String),
}

impl From<String> for ExplorerError {
    fn from(msg: String) -> Self {
        ExplorerError::ApiError(msg)
    }
}

impl From<crate::api::error::ApiError> for ExplorerError {
    fn from(err: crate::api::error::ApiError) -> Self {
        ExplorerError::ApiError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ExplorerError>;
