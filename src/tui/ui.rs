// UI rendering for TUI

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap, Clear},
    Frame,
};

use crate::tui::app::{App, AppMode, ViewState};

pub fn draw(frame: &mut Frame, app: &mut App) {
    // Set black background for entire terminal
    frame.render_widget(Clear, frame.area());
    frame.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black)),
        frame.area(),
    );
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Input box (or status)
            Constraint::Length(1), // Footer
        ])
        .split(frame.area());

    draw_header(frame, chunks[0], app);
    draw_main_content(frame, chunks[1], app);
    draw_input_or_status(frame, chunks[2], app);
    draw_footer(frame, chunks[3], app);
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let inner_area = header_block.inner(area);
    frame.render_widget(header_block, area);

    // Split inner area into three parts: left (price), center (ChainX), right (chain name)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(14), // Left: ETH price "ETH: $0000.00"
            Constraint::Min(0),     // Center: ChainX
            Constraint::Length(12), // Right: "● Ethereum" with padding
        ])
        .split(inner_area);

    // Left: ETH Price
    let eth_price = app.get_eth_price();
    let price_text = if eth_price > 0.0 {
        format!("ETH: ${:.2}", eth_price)
    } else {
        "ETH: ---".to_string()
    };
    let price_widget = Paragraph::new(Span::styled(price_text, Style::default().fg(Color::Green)))
        .alignment(Alignment::Left);
    frame.render_widget(price_widget, chunks[0]);

    // Center: ChainX
    let chainx = Paragraph::new(Span::styled(
        "ChainX",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(chainx, chunks[1]);

    // Right: Green dot + Ethereum
    let chain_indicator = Paragraph::new(Line::from(vec![
        Span::styled("●", Style::default().fg(Color::Green)),
        Span::raw(" Ethereum"),
    ]))
    .alignment(Alignment::Right);
    frame.render_widget(chain_indicator, chunks[2]);
}

fn draw_main_content(frame: &mut Frame, area: Rect, app: &mut App) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Black))
        .title(match &app.view_state {
            ViewState::Home => " Welcome ",
            ViewState::Transaction(_) => " Transaction Details ",
            ViewState::Block(_) => " Block Details ",
            ViewState::Address(_) => " Address Details ",
            ViewState::Contract(_) => " Contract Details ",
            ViewState::Error(_) => " Error ",
        })
        .title_alignment(Alignment::Left);

    let inner_area = content_block.inner(area);
    frame.render_widget(content_block, area);

    match &app.view_state {
        ViewState::Home => draw_home_view(frame, inner_area),
        ViewState::Transaction(tx) => super::widgets::transaction::draw_transaction_widget(
            frame,
            inner_area,
            tx,
            app.scroll_offset,
        ),
        ViewState::Block(block) => {
            super::widgets::block::draw_block_widget(frame, inner_area, block, app.scroll_offset)
        }
        ViewState::Address(addr) => {
            super::widgets::address::draw_address_widget(frame, inner_area, addr, app.scroll_offset)
        }
        ViewState::Contract(contract) => super::widgets::contract::draw_contract_widget(
            frame,
            inner_area,
            contract,
            app.scroll_offset,
        ),
        ViewState::Error(msg) => draw_error_view(frame, inner_area, msg),
    }

    // Draw scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state = ScrollbarState::default()
        .position(app.scroll_offset as usize)
        .content_length(1000); // Dynamic based on content

    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            horizontal: 0,
            vertical: 1,
        }),
        &mut scrollbar_state,
    );
}

fn draw_home_view(frame: &mut Frame, area: Rect) {
    let home_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to ChainX - Terminal Blockchain Explorer",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Supported Input Types:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  • Transaction Hash: 0x... (66 characters)"),
        Line::from("  • Block Number: 12345678 or 0xABC123"),
        Line::from("  • Address: 0x... (42 characters)"),
        Line::from("  • Contract Address: 0x... (42 characters)"),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Keyboard Shortcuts:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  • i or /    - Enter input mode"),
        Line::from("  • h         - Go to home screen"),
        Line::from("  • q         - Quit application"),
        Line::from("  • ↑/↓ or j/k - Scroll content"),
        Line::from("  • PgUp/PgDn - Fast scroll"),
        Line::from("  • Home/End  - Jump to top/bottom"),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'i' to start exploring the blockchain!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let home_paragraph = Paragraph::new(Text::from(home_text))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::default().bg(Color::Black));

    frame.render_widget(home_paragraph, area);
}

fn draw_error_view(frame: &mut Frame, area: Rect, msg: &str) {
    let error_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "❌ Error Occurred",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(msg),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'h' to return home or 'i' to enter a new query",
            Style::default().fg(Color::Gray),
        )),
    ];

    let error_paragraph = Paragraph::new(Text::from(error_text))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::default().bg(Color::Black));

    frame.render_widget(error_paragraph, area);
}

fn draw_input_or_status(frame: &mut Frame, area: Rect, app: &App) {
    match app.mode {
        AppMode::Input => {
            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Input ")
                .title_alignment(Alignment::Left)
                .style(Style::default().bg(Color::Black));

            let input_text = Paragraph::new(app.input.as_str())
                .block(input_block)
                .style(Style::default().fg(Color::White));

            frame.render_widget(input_text, area);

            // Show cursor
            frame.set_cursor_position(ratatui::layout::Position::new(
                area.x + app.cursor_position as u16 + 1,
                area.y + 1,
            ));
        }
        _ => {
            let status_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
                .style(Style::default().bg(Color::Black));

            let status_text = match app.mode {
                AppMode::Loading => "⏳ Querying blockchain...",
                AppMode::Error => "❌ An error occurred",
                _ => "Press 'i' to search, 'h' for home, 'q' to quit",
            };

            let status_paragraph = Paragraph::new(status_text)
                .block(status_block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));

            frame.render_widget(status_paragraph, area);
        }
    }
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let footer_text = match app.mode {
        AppMode::Input => "Enter: Submit | Esc: Cancel | ↑/↓: History",
        _ => "i: Input | h: Home | q: Quit | ↑↓: Scroll",
    };

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black));

    frame.render_widget(footer, area);
}
