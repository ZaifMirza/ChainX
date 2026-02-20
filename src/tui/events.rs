// Event handling for TUI

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    #[allow(dead_code)]
    tick_rate: Duration,
    last_tick: Instant,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
            last_tick: Instant::now(),
        }
    }

    #[allow(dead_code)]
    pub fn next_event(&self) -> std::io::Result<Event> {
        let timeout = self
            .tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let CrosstermEvent::Key(key) = event::read()? {
                return Ok(Event::Key(key));
            }
        }

        Ok(Event::Tick)
    }

    pub fn update_tick(&mut self) {
        self.last_tick = Instant::now();
    }
}

// Normal mode key handling
pub fn handle_normal_mode(key: KeyEvent) -> Option<AppAction> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(AppAction::Quit),
        KeyCode::Char('i') | KeyCode::Char('I') => Some(AppAction::EnterInput),
        KeyCode::Char('/') => Some(AppAction::EnterInput),
        KeyCode::Char('h') | KeyCode::Char('H') => Some(AppAction::GoHome),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(AppAction::EnterApiKeySetup),
        KeyCode::Up | KeyCode::Char('k') => Some(AppAction::ScrollUp),
        KeyCode::Down | KeyCode::Char('j') => Some(AppAction::ScrollDown),
        KeyCode::PageUp => Some(AppAction::ScrollUpFast),
        KeyCode::PageDown => Some(AppAction::ScrollDownFast),
        KeyCode::Home => Some(AppAction::ScrollToTop),
        KeyCode::End => Some(AppAction::ScrollToBottom),
        _ => None,
    }
}

// Input mode key handling
pub fn handle_input_mode(key: KeyEvent) -> Option<AppAction> {
    match key.code {
        KeyCode::Enter => Some(AppAction::Submit),
        KeyCode::Esc => Some(AppAction::CancelInput),
        KeyCode::Char(c) => Some(AppAction::InsertChar(c)),
        KeyCode::Backspace => Some(AppAction::DeleteChar),
        KeyCode::Left => Some(AppAction::MoveCursorLeft),
        KeyCode::Right => Some(AppAction::MoveCursorRight),
        KeyCode::Home => Some(AppAction::MoveCursorStart),
        KeyCode::End => Some(AppAction::MoveCursorEnd),
        _ => None,
    }
}

// API key setup mode key handling
pub fn handle_api_key_mode(key: KeyEvent) -> Option<AppAction> {
    match key.code {
        KeyCode::Enter => Some(AppAction::SubmitApiKey),
        KeyCode::Esc => Some(AppAction::CancelApiKeySetup),
        KeyCode::Char(c) => Some(AppAction::InsertApiKeyChar(c)),
        KeyCode::Backspace => Some(AppAction::DeleteApiKeyChar),
        KeyCode::Left => Some(AppAction::MoveApiKeyCursorLeft),
        KeyCode::Right => Some(AppAction::MoveApiKeyCursorRight),
        KeyCode::Home => Some(AppAction::MoveApiKeyCursorStart),
        KeyCode::End => Some(AppAction::MoveApiKeyCursorEnd),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum AppAction {
    // Navigation
    Quit,
    GoHome,
    
    // Input mode
    EnterInput,
    CancelInput,
    Submit,
    InsertChar(char),
    DeleteChar,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorStart,
    MoveCursorEnd,
    
    // API key setup mode
    EnterApiKeySetup,
    CancelApiKeySetup,
    SubmitApiKey,
    InsertApiKeyChar(char),
    DeleteApiKeyChar,
    MoveApiKeyCursorLeft,
    MoveApiKeyCursorRight,
    MoveApiKeyCursorStart,
    MoveApiKeyCursorEnd,
    
    // Scrolling
    ScrollUp,
    ScrollDown,
    ScrollUpFast,
    ScrollDownFast,
    ScrollToTop,
    ScrollToBottom,
}
