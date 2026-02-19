// Chain configuration types

use crate::error::Result;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub name: &'static str,
    pub chain_id: &'static str,
    pub rpc_url: &'static str,
    pub symbol: &'static str,
}

pub trait ChainResolver {
    fn resolve(chain_name: &str) -> Result<ChainConfig>;
}

pub struct ChainResolverImpl;

impl ChainResolver for ChainResolverImpl {
    fn resolve(chain_name: &str) -> Result<ChainConfig> {
        match chain_name.to_lowercase().as_str() {
            "ethereum" | "eth" | "1" => Ok(ChainConfig {
                name: "Ethereum",
                chain_id: "1",
                rpc_url: "https://ethereum-rpc.publicnode.com",
                symbol: "ETH",
            }),
            "polygon" | "matic" | "137" => Ok(ChainConfig {
                name: "Polygon",
                chain_id: "137",
                rpc_url: "https://polygon-rpc.com",
                symbol: "MATIC",
            }),
            "bsc" | "binance" | "56" => Ok(ChainConfig {
                name: "BSC",
                chain_id: "56",
                rpc_url: "https://bsc-dataseed1.binance.org",
                symbol: "BNB",
            }),
            "avalanche" | "avax" | "43114" => Ok(ChainConfig {
                name: "Avalanche",
                chain_id: "43114",
                rpc_url: "https://api.avax.network/ext/bc/C/rpc",
                symbol: "AVAX",
            }),
            "arbitrum" | "arb" | "42161" => Ok(ChainConfig {
                name: "Arbitrum",
                chain_id: "42161",
                rpc_url: "https://arb1.arbitrum.io/rpc",
                symbol: "ETH",
            }),
            "optimism" | "op" | "10" => Ok(ChainConfig {
                name: "Optimism",
                chain_id: "10",
                rpc_url: "https://mainnet.optimism.io",
                symbol: "ETH",
            }),
            "base" | "8453" => Ok(ChainConfig {
                name: "Base",
                chain_id: "8453",
                rpc_url: "https://mainnet.base.org",
                symbol: "ETH",
            }),
            "celo" | "42220" => Ok(ChainConfig {
                name: "Celo",
                chain_id: "42220",
                rpc_url: "https://forno.celo.org",
                symbol: "CELO",
            }),
            "fantom" | "ftm" | "250" => Ok(ChainConfig {
                name: "Fantom",
                chain_id: "250",
                rpc_url: "https://rpc.fantom.network",
                symbol: "FTM",
            }),
            "goerli" | "5" => Ok(ChainConfig {
                name: "Goerli",
                chain_id: "5",
                rpc_url: "https://ethereum-goerli-rpc.publicnode.com",
                symbol: "ETH",
            }),
            "sepolia" | "11155111" => Ok(ChainConfig {
                name: "Sepolia",
                chain_id: "11155111",
                rpc_url: "https://ethereum-sepolia-rpc.publicnode.com",
                symbol: "ETH",
            }),
            _ => Ok(ChainConfig {
                name: "Ethereum",
                chain_id: "1",
                rpc_url: "https://ethereum-rpc.publicnode.com",
                symbol: "ETH",
            }),
        }
    }
}

pub fn get_chain(chain_name: &str) -> ChainConfig {
    ChainResolverImpl::resolve(chain_name).unwrap_or_else(|_| ChainConfig {
        name: "Ethereum",
        chain_id: "1",
        rpc_url: "https://ethereum-rpc.publicnode.com",
        symbol: "ETH",
    })
}
