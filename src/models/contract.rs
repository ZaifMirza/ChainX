// Contract models

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize)]
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
    #[serde(rename = "functionName")]
    pub function_name: String,
}
