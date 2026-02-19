// RPC client implementation

use reqwest::Client;
use crate::api::rpc::{error, request};
use crate::api::error::ApiResult;
use crate::models::{TransactionDetail, TransactionReceipt, BlockInfo, RpcResponse};

pub struct RpcClient {
    client: Client,
    rpc_url: String,
}

impl RpcClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.to_string(),
        }
    }

    pub async fn execute(&self, request: serde_json::Value) -> ApiResult<RpcResponse> {
        let response = self
            .client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| crate::api::error::ApiError::HttpError(e.to_string()))?;

        response
            .json::<RpcResponse>()
            .await
            .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))
    }

    pub async fn get_balance(&self, address: &str) -> ApiResult<String> {
        let request = request::get_balance_request(address);
        let response = self.execute(request).await?;
        
        let result: String = error::handle_rpc_response(response.result, response.error)?;
        Ok(result)
    }

    pub async fn get_transaction(&self, tx_hash: &str) -> ApiResult<TransactionDetail> {
        let request = request::get_transaction_request(tx_hash);
        let response = self.execute(request).await?;
        
        error::handle_rpc_response(response.result, response.error)
    }

    pub async fn get_transaction_receipt(&self, tx_hash: &str) -> ApiResult<Option<TransactionReceipt>> {
        let request = request::get_receipt_request(tx_hash);
        let response = self.execute(request).await?;
        
        if let Some(ref err) = response.error {
            return Err(crate::api::error::ApiError::RpcError(format!(
                "{} ({})",
                err.message, err.code
            )));
        }

        match response.result {
            Some(val) if val.is_null() => Ok(None),
            Some(val) => {
                let receipt: TransactionReceipt = serde_json::from_value(val)
                    .map_err(|e| crate::api::error::ApiError::ParseError(e.to_string()))?;
                Ok(Some(receipt))
            }
            None => Ok(None),
        }
    }

    pub async fn get_block(&self, block_number: u64, full_transactions: bool) -> ApiResult<BlockInfo> {
        let request = request::get_block_request(block_number, full_transactions);
        let response = self.execute(request).await?;
        
        error::handle_rpc_response(response.result, response.error)
    }

    pub async fn get_block_timestamp(&self, block_number: u64) -> ApiResult<Option<String>> {
        let block = self.get_block(block_number, false).await?;
        Ok(block.timestamp)
    }

    pub async fn fetch_transaction_data(
        &self,
        tx_hash: &str,
    ) -> ApiResult<(TransactionDetail, Option<TransactionReceipt>, Option<String>)> {
        let tx_detail = self.get_transaction(tx_hash).await?;
        let receipt = self.get_transaction_receipt(tx_hash).await?;
        
        let block_number = crate::utils::hex::parse_hex(
            tx_detail.block_number.as_deref().unwrap_or("0x0")
        );
        
        let timestamp = if block_number > 0 {
            self.get_block_timestamp(block_number).await.ok().flatten()
        } else {
            None
        };

        Ok((tx_detail, receipt, timestamp))
    }

    pub async fn get_block_number(&self) -> ApiResult<u64> {
        let request = request::get_block_number_request();
        let response = self.execute(request).await?;
        
        let result: String = error::handle_rpc_response(response.result, response.error)?;
        Ok(crate::utils::hex::parse_hex(&result))
    }

    pub async fn get_transaction_count(&self, address: &str) -> ApiResult<u64> {
        let request = request::get_transaction_count_request(address);
        let response = self.execute(request).await?;
        
        let result: String = error::handle_rpc_response(response.result, response.error)?;
        Ok(crate::utils::hex::parse_hex(&result))
    }
}
