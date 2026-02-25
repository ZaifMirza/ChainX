// API error types
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("RPC Error: {0}")]
    RpcError(String),
    #[error("HTTP Error: {0}")]
    HttpError(String),
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[allow(dead_code)]
    #[error("Not Found: {0}")]
    NotFound(String),
    #[allow(dead_code)]
    #[error("Rate Limited: {0}")]
    RateLimited(String),
}

pub type ApiResult<T> = Result<T, ApiError>;
