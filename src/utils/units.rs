// Unit conversion utilities

use crate::config::constants::{WEI_PER_ETH, WEI_PER_GWEI};
use crate::utils::hex::parse_hex_u128;

pub fn wei_to_eth(wei: &str) -> f64 {
    let wei_val = parse_hex_u128(wei);
    wei_val as f64 / WEI_PER_ETH
}

#[allow(dead_code)]
pub fn wei_to_gwei(wei: &str) -> f64 {
    let wei_val = parse_hex_u128(wei);
    wei_val as f64 / WEI_PER_GWEI
}

pub fn parse_token_balance(balance: &str, decimals: u32) -> f64 {
    let balance_val = parse_hex_u128(balance);
    let divisor = 10_f64.powi(decimals as i32);
    balance_val as f64 / divisor
}

#[allow(dead_code)]
pub fn format_large_number(n: f64) -> String {
    if n > 1_000_000.0 {
        format!("{:.2e}", n)
    } else {
        format!("{:.8}", n)
    }
}
