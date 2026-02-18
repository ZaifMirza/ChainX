// Token display formatter

use crate::config::constants::MAX_TOKEN_DISPLAY;
use crate::models::{TokenInfo, TokenTransfer};
use crate::utils::units::format_large_number;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct TokenListFormatter<'a> {
    transfers: &'a [TokenTransfer],
}

impl<'a> TokenListFormatter<'a> {
    #[allow(dead_code)]
    pub fn new(transfers: &'a [TokenTransfer]) -> Self {
        Self { transfers }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> Option<String> {
        if self.transfers.is_empty() {
            return None;
        }

        let mut seen: HashSet<String> = HashSet::new();
        let mut unique_tokens: Vec<&TokenTransfer> = Vec::new();
        let mut filtered_count = 0;

        for t in self.transfers {
            if t.is_suspicious() {
                filtered_count += 1;
                continue;
            }
            if !seen.contains(&t.contract_address) {
                seen.insert(t.contract_address.clone());
                unique_tokens.push(t);
            }
            if unique_tokens.len() >= MAX_TOKEN_DISPLAY {
                break;
            }
        }

        if unique_tokens.is_empty() && filtered_count == 0 {
            return None;
        }

        let mut output = String::new();

        if !unique_tokens.is_empty() {
            output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
            output.push_str("║  RECENT TOKEN TRANSFERS (Filtered)                        ║\n");
            output.push_str("╠═══════════════════════════════════════════════════════════╣\n");

            for token in &unique_tokens {
                let value = token.display_value();
                let symbol = token.display_symbol();
                let symbol_display = crate::utils::text::truncate_for_display(&symbol, 10);
                let value_display = format_large_number(value);
                output.push_str(&format!(
                    "║  {:>18} │ {:>20} ║\n",
                    symbol_display, value_display
                ));
            }
        }

        let remaining = self.transfers.len() - unique_tokens.len() - filtered_count;
        if remaining > 0 || filtered_count > 0 {
            output.push_str(&format!(
                "║  {:>18} │ {:>20} ║\n",
                if filtered_count > 0 {
                    "⚠️ Scams filtered"
                } else {
                    ""
                },
                format!("... {} more", remaining)
            ));
        }

        Some(output)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Some(formatted) = self.format() {
            print!("{}", formatted);
        }
    }
}

#[allow(dead_code)]
pub fn print_token_transfers(transfers: &[TokenTransfer]) {
    let formatter = TokenListFormatter::new(transfers);
    formatter.print();
}
