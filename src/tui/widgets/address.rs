// Address detail widget

use ratatui::{
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::models::AddressDisplay;
use crate::tui::widgets::common::*;

pub fn draw_address_widget(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    addr: &AddressDisplay,
    scroll_offset: u16,
) {
    let mut lines = vec![];

    // Address
    lines.push(key_value_line("Address:", &addr.address));
    lines.push(Line::from(""));

    // Type
    let type_text = if addr.is_contract {
        "Smart Contract"
    } else {
        "EOA (Wallet)"
    };
    let type_color = if addr.is_contract {
        Color::Magenta
    } else {
        Color::Cyan
    };
    lines.push(key_value_line_colored("Type:", type_text, type_color));
    lines.push(Line::from(""));

    // Balance
    lines.push(section_title("Balance"));
    lines.push(key_value_line_colored(
        "ETH Balance:",
        &format!("{} ETH", addr.balance_eth),
        Color::Green,
    ));

    if let Some(ref usd_value) = addr.usd_value {
        lines.push(key_value_line("USD Value:", usd_value));
    }
    lines.push(Line::from(""));

    // Transaction Count
    lines.push(key_value_line(
        "Transaction Count:",
        &addr.transaction_count.to_string(),
    ));
    lines.push(Line::from(""));

    // Contract Info
    if addr.is_contract {
        if let Some(ref contract_name) = addr.contract_name {
            lines.push(section_title("Contract Information"));
            lines.push(key_value_line("Name:", contract_name));

            if let Some(ref creator) = addr.contract_creator {
                lines.push(key_value_line("Creator:", creator));
            }

            if let Some(ref creation_tx) = addr.creation_transaction {
                lines.push(key_value_line("Creation Tx:", creation_tx));
            }

            lines.push(Line::from(""));
        }
    }

    // Token Balances
    if !addr.token_balances.is_empty() {
        lines.push(section_title("Token Balances"));

        for token in &addr.token_balances {
            let symbol = token.symbol.as_deref().unwrap_or("???");
            let balance = &token.balance_formatted;
            lines.push(Line::from(format!("  • {} {}", balance, symbol)));
        }

        lines.push(Line::from(""));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}
