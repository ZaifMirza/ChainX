// Contract detail widget

use ratatui::{
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::models::ContractDisplay;
use crate::tui::widgets::common::*;

pub fn draw_contract_widget(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    contract: &ContractDisplay,
    scroll_offset: u16,
) {
    let mut lines = vec![];

    // Contract Address
    lines.push(key_value_line("Contract Address:", &contract.address));
    lines.push(Line::from(""));

    // Balance
    lines.push(section_title("Balance"));
    lines.push(key_value_line_colored(
        "ETH Balance:",
        format!("{} ETH", contract.balance_eth),
        Color::Green,
    ));

    if let Some(ref usd_value) = contract.usd_value {
        lines.push(key_value_line("USD Value:", usd_value));
    }
    lines.push(Line::from(""));

    // Contract Info
    if let Some(ref name) = contract.name {
        lines.push(section_title("Contract Information"));
        lines.push(key_value_line("Name:", name));

        if let Some(ref symbol) = contract.symbol {
            lines.push(key_value_line("Symbol:", symbol));
        }

        if let Some(ref creator) = contract.creator {
            lines.push(key_value_line("Creator:", creator));
        }

        if let Some(ref creation_tx) = contract.creation_transaction {
            lines.push(key_value_line("Creation Tx:", creation_tx));
        }

        if let Some(ref compiler) = contract.compiler {
            lines.push(key_value_line("Compiler:", compiler));
        }

        if contract.is_proxy {
            lines.push(key_value_line_colored(
                "Type:",
                "Proxy Contract",
                Color::Magenta,
            ));
            if let Some(ref impl_addr) = contract.implementation {
                lines.push(key_value_line("Implementation:", impl_addr));
            }
        }

        lines.push(Line::from(""));
    }

    // Transaction Count
    lines.push(key_value_line(
        "Total Transactions:",
        contract.transaction_count.to_string(),
    ));
    lines.push(Line::from(""));

    // Recent Transactions
    if !contract.recent_transactions.is_empty() {
        lines.push(section_title("Recent Transactions"));

        for (i, tx) in contract.recent_transactions.iter().enumerate() {
            lines.push(Line::from(format!("  #{} {}", i + 1, "─".repeat(40))));
            lines.push(Line::from(format!(
                "    Hash: {}",
                truncate_address(&tx.hash)
            )));
            lines.push(Line::from(format!(
                "    From: {}",
                truncate_address(&tx.from)
            )));
            lines.push(Line::from(format!("    To: {}", truncate_address(&tx.to))));
            lines.push(Line::from(format!("    Value: {} ETH", tx.value_eth())));
            lines.push(Line::from(format!(
                "    Status: {}",
                if tx.is_success() {
                    "✅ Success"
                } else {
                    "❌ Failed"
                }
            )));
            lines.push(Line::from(""));
        }
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}
