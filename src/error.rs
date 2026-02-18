// Error types for the blockchain explorer

use std::fmt;

#[derive(Debug)]
pub enum ExplorerError {
    RpcError(String),
    ApiError(String),
    #[allow(dead_code)]
    ParseError(String),
    ValidationError(String),
    ConfigError(String),
    #[allow(dead_code)]
    CacheError(String),
}

impl fmt::Display for ExplorerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplorerError::RpcError(msg) => write!(f, "RPC Error: {}", msg),
            ExplorerError::ApiError(msg) => write!(f, "API Error: {}", msg),
            ExplorerError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            ExplorerError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            ExplorerError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            ExplorerError::CacheError(msg) => write!(f, "Cache Error: {}", msg),
        }
    }
}

impl std::error::Error for ExplorerError {}

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
