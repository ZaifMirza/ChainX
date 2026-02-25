// Application constants

#[allow(dead_code)]
pub const APP_NAME: &str = "ChainX";
#[allow(dead_code)]
pub const APP_VERSION: &str = "0.1.0";

// Display constants
#[allow(dead_code)]
pub const DISPLAY_WIDTH: usize = 59;
#[allow(dead_code)]
pub const TRUNCATE_LENGTH: usize = 28;
pub const BOX_WIDTH: usize = 61;

// Ethereum constants
pub const WEI_PER_ETH: f64 = 1e18;
#[allow(dead_code)]
pub const WEI_PER_GWEI: f64 = 1e9;
pub const ETH_ADDRESS_LENGTH: usize = 42;
#[allow(dead_code)]
pub const TX_HASH_LENGTH: usize = 66;

// Cache constants
pub const PRICE_CACHE_DURATION_SECS: u64 = 5;
pub const MAX_TOKEN_DISPLAY: usize = 10;

// Block reward constants
pub const LONDON_BLOCK: u64 = 15537393;
pub const CONSTANTINOPLE_BLOCK: u64 = 7280000;
pub const BYZANTIUM_BLOCK: u64 = 4370000;

// Base rewards in ETH
pub const BASE_REWARD_LONDON: f64 = 0.0;
pub const BASE_REWARD_CONSTANTINOPLE: f64 = 2.0;
pub const BASE_REWARD_BYZANTIUM: f64 = 3.0;
pub const BASE_REWARD_GENESIS: f64 = 5.0;

// Token filter constants
#[allow(dead_code)]
pub const MAX_TOKEN_SYMBOL_LENGTH: usize = 20;
#[allow(dead_code)]
pub const MAX_TOKEN_NAME_LENGTH: usize = 50;
