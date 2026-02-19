// Event handling for TUI

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use std::time::{Duration, Instant};

pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
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

pub fn handle_normal_mode(key: KeyEvent) -> Option<AppAction> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(AppAction::Quit),
        KeyCode::Char('i') | KeyCode::Char('I') => Some(AppAction::EnterInput),
        KeyCode::Char('/') => Some(AppAction::EnterInput),
        KeyCode::Char('h') | KeyCode::Char('H') => Some(AppAction::GoHome),
        KeyCode::Up | KeyCode::Char('k') => Some(AppAction::ScrollUp),
        KeyCode::Down | KeyCode::Char('j') => Some(AppAction::ScrollDown),
        KeyCode::PageUp => Some(AppAction::ScrollUpFast),
        KeyCode::PageDown => Some(AppAction::ScrollDownFast),
        KeyCode::Home => Some(AppAction::ScrollToTop),
        KeyCode::End => Some(AppAction::ScrollToBottom),
        _ => None,
    }
}

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

#[derive(Debug, Clone)]
pub enum AppAction {
    Quit,
    EnterInput,
    CancelInput,
    Submit,
    InsertChar(char),
    DeleteChar,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorStart,
    MoveCursorEnd,
    ScrollUp,
    ScrollDown,
    ScrollUpFast,
    ScrollDownFast,
    ScrollToTop,
    ScrollToBottom,
    GoHome,
}
