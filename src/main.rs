// ChainX - Terminal Blockchain Explorer with TUI
// Main entry point

mod api;
mod api_key;
mod app;
mod cache;
mod commands;
mod config;
mod error;
mod formatting;
mod models;
mod tui;
mod utils;
mod validation;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::io;

use tui::{
    app::{App, AppMode},
    events::{handle_api_key_mode, handle_input_mode, handle_normal_mode, AppAction, EventHandler},
    ui::draw,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            reset_terminal()?;
            eprintln!("Failed to initialize app: {}", e);
            return Err(e.into());
        }
    };

    // Create event handler
    let mut event_handler = EventHandler::new(250); // 250ms tick rate

    // Main loop
    let result = run_app(&mut terminal, &mut app, &mut event_handler).await;

    // Cleanup terminal
    reset_terminal()?;

    // Show any errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn reset_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(DisableMouseCapture)?;
    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &mut EventHandler,
) -> io::Result<()> {
    let mut last_tick = std::time::Instant::now();

    loop {
        // Draw UI
        terminal.draw(|f| draw(f, app))?;

        // Handle events with timeout
        let timeout = calculate_timeout(last_tick);
        
        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                let action = handle_key_event(app.mode, key);
                if let Some(action) = action {
                    handle_action(app, action).await;
                }
            }
        }

        // Handle tick
        if last_tick.elapsed() >= std::time::Duration::from_millis(250) {
            event_handler.update_tick();
            last_tick = std::time::Instant::now();
        }

        if app.should_quit() {
            return Ok(());
        }
    }
}

fn calculate_timeout(last_tick: std::time::Instant) -> std::time::Duration {
    std::time::Duration::from_millis(250)
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| std::time::Duration::from_secs(0))
}

fn handle_key_event(mode: AppMode, key: crossterm::event::KeyEvent) -> Option<AppAction> {
    match mode {
        AppMode::Normal | AppMode::Loading | AppMode::Error => handle_normal_mode(key),
        AppMode::Input => handle_input_mode(key),
        AppMode::ApiKeySetup => handle_api_key_mode(key),
    }
}

async fn handle_action(app: &mut App, action: AppAction) {
    use AppAction::*;

    match action {
        // Navigation
        Quit => app.quit(),
        GoHome => app.go_home(),
        
        // Input mode
        EnterInput => app.enter_input_mode(),
        CancelInput => app.exit_input_mode(),
        Submit => handle_submit(app).await,
        InsertChar(c) => app.insert_char(c),
        DeleteChar => app.delete_char(),
        MoveCursorLeft => app.move_cursor_left(),
        MoveCursorRight => app.move_cursor_right(),
        MoveCursorStart => app.move_cursor_start(),
        MoveCursorEnd => app.move_cursor_end(),
        
        // API key setup mode
        EnterApiKeySetup => app.enter_api_key_setup(),
        CancelApiKeySetup => app.exit_api_key_setup(),
        SubmitApiKey => handle_api_key_submit(app).await,
        InsertApiKeyChar(c) => app.insert_api_key_char(c),
        DeleteApiKeyChar => app.delete_api_key_char(),
        MoveApiKeyCursorLeft => app.move_api_key_cursor_left(),
        MoveApiKeyCursorRight => app.move_api_key_cursor_right(),
        MoveApiKeyCursorStart => app.move_api_key_cursor_start(),
        MoveApiKeyCursorEnd => app.move_api_key_cursor_end(),
        
        // Scrolling
        ScrollUp => app.scroll_up(1),
        ScrollDown => app.scroll_down(1),
        ScrollUpFast => app.scroll_up(5),
        ScrollDownFast => app.scroll_down(5),
        ScrollToTop => app.reset_scroll(),
        ScrollToBottom => app.scroll_down(1000),
    }
}

async fn handle_submit(app: &mut App) {
    if let Err(e) = app.submit_input().await {
        app.view_state = tui::app::ViewState::Error(format!("Error: {}", e));
    }
}

async fn handle_api_key_submit(app: &mut App) {
    if let Err(e) = app.submit_api_key().await {
        app.view_state = tui::app::ViewState::Error(format!("Error: {}", e));
    }
}
