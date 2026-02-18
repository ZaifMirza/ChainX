// RPC request builders

use serde_json::json;

pub struct RpcRequestBuilder {
    method: String,
    params: Vec<serde_json::Value>,
    id: u32,
}

impl RpcRequestBuilder {
    pub fn new(method: &str) -> Self {
        Self {
            method: method.to_string(),
            params: Vec::new(),
            id: 1,
        }
    }

    pub fn param<T: Into<serde_json::Value>>(mut self, value: T) -> Self {
        self.params.push(value.into());
        self
    }

    #[allow(dead_code)]
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> serde_json::Value {
        json!({
            "jsonrpc": "2.0",
            "method": self.method,
            "params": self.params,
            "id": self.id
        })
    }
}

// Pre-built request functions
pub fn get_balance_request(address: &str) -> serde_json::Value {
    RpcRequestBuilder::new("eth_getBalance")
        .param(address)
        .param("latest")
        .build()
}

pub fn get_transaction_request(tx_hash: &str) -> serde_json::Value {
    RpcRequestBuilder::new("eth_getTransactionByHash")
        .param(tx_hash)
        .build()
}

pub fn get_receipt_request(tx_hash: &str) -> serde_json::Value {
    RpcRequestBuilder::new("eth_getTransactionReceipt")
        .param(tx_hash)
        .build()
}

pub fn get_block_request(block_number: u64, full_tx: bool) -> serde_json::Value {
    RpcRequestBuilder::new("eth_getBlockByNumber")
        .param(format!("0x{:x}", block_number))
        .param(full_tx)
        .build()
}
