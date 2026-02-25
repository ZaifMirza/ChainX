// Contract models

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ContractInfo {
    #[serde(rename = "ContractName", default)]
    pub contract_name: String,
    #[serde(rename = "CompilerVersion", default)]
    pub compiler_version: String,
    #[allow(dead_code)]
    #[serde(rename = "SourceCode", default)]
    pub source_code: String,
    #[allow(dead_code)]
    #[serde(rename = "ABI", default)]
    pub abi: String,
    #[allow(dead_code)]
    #[serde(rename = "OptimizationUsed", default)]
    pub optimization_used: String,
    #[allow(dead_code)]
    #[serde(rename = "Runs", default)]
    pub runs: String,
    #[allow(dead_code)]
    #[serde(rename = "ConstructorArguments", default)]
    pub constructor_arguments: String,
    #[allow(dead_code)]
    #[serde(rename = "EVMVersion", default)]
    pub evm_version: String,
    #[allow(dead_code)]
    #[serde(rename = "Library", default)]
    pub library: String,
    #[allow(dead_code)]
    #[serde(rename = "LicenseType", default)]
    pub license_type: String,
    #[serde(rename = "Proxy", default)]
    pub proxy: String,
    #[serde(rename = "Implementation", default)]
    pub implementation: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractCreationInfo {
    #[allow(dead_code)]
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "contractCreator")]
    pub contract_creator: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[allow(dead_code)]
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[allow(dead_code)]
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractTransaction {
    #[allow(dead_code)]
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[allow(dead_code)]
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[allow(dead_code)]
    #[serde(rename = "from")]
    pub from: String,
    #[allow(dead_code)]
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "value")]
    pub value: String,
    #[allow(dead_code)]
    #[serde(rename = "gas")]
    pub gas: String,
    #[allow(dead_code)]
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[allow(dead_code)]
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "txreceipt_status")]
    pub status: String,
    #[allow(dead_code)]
    #[serde(rename = "methodId")]
    pub method_id: String,
    #[allow(dead_code)]
    #[serde(rename = "functionName")]
    pub function_name: String,
}

impl ContractTransaction {
    pub fn value_eth(&self) -> f64 {
        let wei = crate::utils::hex::parse_hex(&self.value);
        wei as f64 / 1e18
    }

    pub fn is_success(&self) -> bool {
        self.status == "1"
    }
}

// Display-ready contract for TUI
#[derive(Debug, Clone)]
pub struct ContractDisplay {
    pub address: String,
    pub balance_eth: f64,
    pub usd_value: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub creator: Option<String>,
    pub creation_transaction: Option<String>,
    pub compiler: Option<String>,
    pub is_proxy: bool,
    pub implementation: Option<String>,
    pub transaction_count: usize,
    pub recent_transactions: Vec<ContractTransaction>,
}

impl ContractDisplay {
    pub fn new(address: String) -> Self {
        Self {
            address,
            balance_eth: 0.0,
            usd_value: None,
            name: None,
            symbol: None,
            creator: None,
            creation_transaction: None,
            compiler: None,
            is_proxy: false,
            implementation: None,
            transaction_count: 0,
            recent_transactions: Vec::new(),
        }
    }
}
