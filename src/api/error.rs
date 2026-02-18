// API error types

#[derive(Debug)]
pub enum ApiError {
    RpcError(String),
    HttpError(String),
    ParseError(String),
    #[allow(dead_code)]
    NotFound(String),
    #[allow(dead_code)]
    RateLimited(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::RpcError(msg) => write!(f, "RPC Error: {}", msg),
            ApiError::HttpError(msg) => write!(f, "HTTP Error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::RateLimited(msg) => write!(f, "Rate Limited: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;
