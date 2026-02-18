// RPC client error handling

use crate::api::error::ApiError;

pub fn handle_rpc_response<T>(
    result: Option<serde_json::Value>,
    error: Option<crate::models::rpc::RpcError>,
) -> Result<T, ApiError>
where
    T: serde::de::DeserializeOwned,
{
    if let Some(err) = error {
        return Err(ApiError::RpcError(format!(
            "{} ({})",
            err.message, err.code
        )));
    }

    match result {
        Some(val) if val.is_null() => Err(ApiError::NotFound("Resource not found".to_string())),
        Some(val) => serde_json::from_value(val).map_err(|e| ApiError::ParseError(e.to_string())),
        None => Err(ApiError::RpcError("No result returned".to_string())),
    }
}
