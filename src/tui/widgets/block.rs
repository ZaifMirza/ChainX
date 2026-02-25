// Block detail widget

use ratatui::{
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::models::BlockDisplay;
use crate::tui::widgets::common::*;

pub fn draw_block_widget(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    block: &BlockDisplay,
    scroll_offset: u16,
) {
    let mut lines = vec![];

    // Block Number
    lines.push(key_value_line("Block Number:", block.number.to_string()));
    lines.push(Line::from(""));

    // Status
    let status_text = if block.confirmed {
        "Confirmed"
    } else {
        "Pending"
    };
    let status_color_val = if block.confirmed {
        Color::Green
    } else {
        Color::Yellow
    };
    lines.push(key_value_line_colored(
        "Status:",
        status_text,
        status_color_val,
    ));
    lines.push(Line::from(""));

    // Timestamps
    lines.push(key_value_line("Timestamp:", &block.timestamp));
    lines.push(key_value_line("Age:", &block.age));
    lines.push(Line::from(""));

    // Block Stats
    lines.push(section_title("Block Statistics"));
    lines.push(key_value_line(
        "Transactions:",
        block.transactions.to_string(),
    ));
    if let Some(withdrawals) = block.withdrawals {
        lines.push(key_value_line("Withdrawals:", withdrawals.to_string()));
    }
    lines.push(Line::from(""));

    // Gas Info
    lines.push(section_title("Gas Information"));
    lines.push(key_value_line("Gas Used:", &block.gas_used));
    if let Some(ref gas_percentage) = block.gas_percentage {
        lines.push(key_value_line("Gas %:", gas_percentage));
    }
    lines.push(key_value_line("Gas Limit:", &block.gas_limit));
    lines.push(Line::from(""));

    // Block Rewards
    if let Some(ref reward) = block.block_reward {
        lines.push(key_value_line_colored(
            "Block Reward:",
            reward,
            Color::Green,
        ));
        lines.push(Line::from(""));
    }

    // Hashes
    lines.push(section_title("Block Hashes"));
    lines.push(key_value_line("Block Hash:", &block.hash));
    lines.push(key_value_line("Parent Hash:", &block.parent_hash));
    if let Some(ref state_root) = block.state_root {
        lines.push(key_value_line("State Root:", state_root));
    }
    lines.push(Line::from(""));

    // Miner
    lines.push(key_value_line("Miner:", &block.miner));

    if let Some(ref extra_data) = block.extra_data {
        lines.push(Line::from(""));
        lines.push(section_title("Extra Data"));
        lines.push(Line::from(extra_data.as_str()));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}
