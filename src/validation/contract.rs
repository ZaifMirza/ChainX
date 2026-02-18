// Contract detection validation

use crate::api::RpcClient;

pub struct ContractDetector<'a> {
    client: &'a RpcClient,
    address: &'a str,
}

impl<'a> ContractDetector<'a> {
    pub fn new(client: &'a RpcClient, address: &'a str) -> Self {
        Self { client, address }
    }

    pub async fn is_contract(&self) -> bool {
        match self.get_code().await {
            Ok(code) => !code.is_empty() && code != "0x",
            Err(_) => false,
        }
    }

    async fn get_code(&self) -> Result<String, String> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getCode",
            "params": [self.address, "latest"],
            "id": 1
        });

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(error) = response.error {
            return Err(format!("{} ({})", error.message, error.code));
        }

        if let Some(result) = response.result {
            Ok(result.as_str().unwrap_or("0x").to_string())
        } else {
            Err("No result returned".to_string())
        }
    }
}
