// RPC-specific models

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}
