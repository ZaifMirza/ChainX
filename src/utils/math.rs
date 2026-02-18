// Math and calculation utilities

use crate::config::constants::{
    BASE_REWARD_BYZANTIUM, BASE_REWARD_CONSTANTINOPLE, BASE_REWARD_GENESIS, BASE_REWARD_LONDON,
};
use crate::config::constants::{BYZANTIUM_BLOCK, CONSTANTINOPLE_BLOCK, LONDON_BLOCK};

pub fn calculate_block_reward(block_number: u64, uncle_count: usize) -> f64 {
    let base_reward = if block_number >= LONDON_BLOCK {
        BASE_REWARD_LONDON
    } else if block_number >= CONSTANTINOPLE_BLOCK {
        BASE_REWARD_CONSTANTINOPLE
    } else if block_number >= BYZANTIUM_BLOCK {
        BASE_REWARD_BYZANTIUM
    } else {
        BASE_REWARD_GENESIS
    };

    let uncle_reward: f64 = (0..uncle_count)
        .map(|i| (8 - i) as f64 * base_reward / 8.0)
        .sum();

    base_reward + uncle_reward
}

#[allow(dead_code)]
pub fn calculate_percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (part as f64 / total as f64) * 100.0
}
