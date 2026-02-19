// Transaction detail widget

use ratatui::{
    style::Color,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::models::TransactionDisplay;
use crate::tui::widgets::common::*;

pub fn draw_transaction_widget(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    tx: &TransactionDisplay,
    scroll_offset: u16,
) {
    let mut lines = vec![];

    // Transaction Hash
    lines.push(key_value_line("Hash:", &tx.hash));
    lines.push(Line::from(""));

    // Status
    let status_symbol = if tx.status { "✅" } else { "❌" };
    let status_text = if tx.status { "Success" } else { "Failed" };
    lines.push(key_value_line_colored(
        "Status:",
        &format!("{} {}", status_symbol, status_text),
        status_color(tx.status),
    ));
    lines.push(Line::from(""));

    // Block Info
    lines.push(key_value_line("Block:", &tx.block_number.to_string()));
    lines.push(key_value_line(
        "Confirmations:",
        &tx.confirmations.to_string(),
    ));
    lines.push(key_value_line("Timestamp:", &tx.timestamp));
    lines.push(Line::from(""));

    // From/To
    lines.push(key_value_line("From:", &tx.from));
    lines.push(key_value_line("To:", &tx.to));
    lines.push(Line::from(""));

    // Value
    lines.push(key_value_line_colored(
        "Value:",
        &format!("{} ETH", tx.value_eth),
        Color::Green,
    ));
    lines.push(Line::from(""));

    // Gas Info
    lines.push(section_title("Gas Information"));
    lines.push(key_value_line("Gas Used:", &tx.gas_used));
    lines.push(key_value_line(
        "Gas Price:",
        &format!("{} Gwei", tx.gas_price_gwei),
    ));
    lines.push(key_value_line_colored(
        "Transaction Fee:",
        &format!("{} ETH", tx.transaction_fee_eth),
        Color::Yellow,
    ));
    lines.push(Line::from(""));

    // Nonce
    lines.push(key_value_line("Nonce:", &tx.nonce.to_string()));

    if let Some(ref input) = tx.input_data {
        if !input.is_empty() && input != "0x" {
            lines.push(Line::from(""));
            lines.push(section_title("Input Data"));
            lines.push(Line::from(input.as_str()));
        }
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0));

    frame.render_widget(paragraph, area);
}
