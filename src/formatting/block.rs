// Block display formatter

use crate::models::BlockInfo;
use crate::models::BlockStats;
use crate::utils::hex::parse_hex;
use crate::utils::text::truncate_and_pad;
use crate::utils::{calculate_block_reward, format_timestamp};

#[allow(dead_code)]
pub struct BlockFormatter<'a> {
    block: &'a BlockInfo,
    symbol: &'a str,
}

impl<'a> BlockFormatter<'a> {
    #[allow(dead_code)]
    pub fn new(block: &'a BlockInfo, symbol: &'a str) -> Self {
        Self { block, symbol }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let mut output = String::new();

        let number = parse_hex(self.block.number.as_deref().unwrap_or("0x0"));
        let tx_count = self.block.transaction_count();
        let gas_percent = self.block.gas_usage_percent();
        let block_reward = calculate_block_reward(number, self.block.uncle_count());

        output.push_str("╔═══════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    BLOCK DETAILS                          ║\n");
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!("║  {:18} │ {}  ║\n", "Block Height:", number));
        output.push_str(&format!("║  {:18} │ {}  ║\n", "Status:", "Confirmed"));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Timestamp:",
            truncate_and_pad(&format_timestamp(self.block.timestamp.as_deref()), 28)
        ));
        output.push_str(&format!("║  {:18} │ {}  ║\n", "Transactions:", tx_count));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Withdrawals:",
            self.block.withdrawal_count()
        ));
        output.push_str(&format!(
            "║  {:18} │ {} {} ║\n",
            "Block Reward:", block_reward, self.symbol
        ));
        output.push_str(&format!(
            "║  {:18} │ {} ({:.1}%)  ║\n",
            "Gas Used:",
            parse_hex(self.block.gas_used.as_deref().unwrap_or("0x0")),
            gas_percent
        ));
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!(
            "║  Block Hash:   {}  ║\n",
            truncate_and_pad(self.block.hash.as_deref().unwrap_or("N/A"), 50)
        ));
        output.push_str(&format!(
            "║  Parent Hash:  {}  ║\n",
            truncate_and_pad(self.block.parent_hash.as_deref().unwrap_or("N/A"), 50)
        ));
        output.push_str(&format!(
            "║  Miner:       {}  ║\n",
            truncate_and_pad(self.block.miner.as_deref().unwrap_or("N/A"), 50)
        ));
        output.push_str(&format!(
            "║  Gas Limit:   {}  ║\n",
            parse_hex(self.block.gas_limit.as_deref().unwrap_or("0x1"))
        ));
        output.push_str(&format!(
            "║  Difficulty:  {}  ║\n",
            self.block.difficulty.as_deref().unwrap_or("N/A")
        ));
        output.push_str("╚═══════════════════════════════════════════════════════════╝");

        output
    }

    pub fn print(&self) {
        println!("{}", self.format());

        let tx_count = self.block.transaction_count();
        if tx_count > 0 && tx_count <= 10 {
            println!("\n📝 Transaction Hashes:");
            if let Some(txs) = self.block.transactions.as_ref() {
                for tx in txs {
                    if let Some(hash) = tx.as_str() {
                        println!("  ↳ {}", hash);
                    }
                }
            }
        } else if tx_count > 10 {
            println!("\n📝 ({} total transactions)", tx_count);
        }
    }
}

pub fn print_block_details(block: &BlockInfo, symbol: &str) {
    let formatter = BlockFormatter::new(block, symbol);
    formatter.print();
}
