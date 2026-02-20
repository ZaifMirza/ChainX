// Common widget utilities

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn key_value_line(key: &str, value: impl Into<String>) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:20}", key), Style::default().fg(Color::Yellow)),
        Span::raw(value.into()),
    ])
}

pub fn key_value_line_colored(
    key: &str,
    value: impl Into<String>,
    value_color: Color,
) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:20}", key), Style::default().fg(Color::Yellow)),
        Span::styled(value.into(), Style::default().fg(value_color)),
    ])
}

pub fn section_title(title: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw(""),
        Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(""),
    ])
}

pub fn truncate_address(address: &str) -> String {
    if address.len() > 20 {
        format!("{}...{}", &address[..10], &address[address.len() - 8..])
    } else {
        address.to_string()
    }
}

pub fn status_color(success: bool) -> Color {
    if success {
        Color::Green
    } else {
        Color::Red
    }
}

#[allow(dead_code)]
pub fn status_symbol(success: bool) -> &'static str {
    if success {
        "✅"
    } else {
        "❌"
    }
}
