//! TUI Application state and logic

use crate::app::{AppConfig, AppState, Result};
use crate::commands::CommandRouter;
use crate::models::{AddressDisplay, BlockDisplay, ContractDisplay, TransactionDisplay};
use crate::app::input::InputParser;
use super::terminal::AppTerminal;
use super::events::{EventHandler, handle_api_key_mode, handle_input_mode, handle_normal_mode, AppAction};
use super::ui::draw;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::time::interval;

const TICK_RATE_MS: u64 = 250;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    ApiKeySetup,
    Loading,
    #[allow(dead_code)]
    Error,
}

#[derive(Debug, Clone)]
pub enum ViewState {
    Home,
    Transaction(Box<TransactionDisplay>),
    Block(Box<BlockDisplay>),
    Address(Box<AddressDisplay>),
    Contract(Box<ContractDisplay>),
    Error(String),
    ApiKeyRequired(String),
}

/// Represents the price state - None means loading, Some(price) means loaded
pub type PriceState = Option<f64>;

pub struct App {
    pub mode: AppMode,
    pub view_state: ViewState,
    pub input: String,
    pub cursor_position: usize,
    pub state: AppState,
    pub scroll_offset: u16,
    pub should_quit: bool,
    pub eth_price: Arc<RwLock<PriceState>>,
    pub price_loading: Arc<AtomicBool>,
    pub api_key_input: String,
    pub api_key_cursor: usize,
}

impl App {
    pub async fn new() -> Result<Self> {
        let app_config = AppConfig::load()?;
        let state = AppState::new(app_config);
        let eth_price = Arc::new(RwLock::new(None));
        let price_loading = Arc::new(AtomicBool::new(false));

        if let Some(api_key) = state.config.etherscan_api_key.clone() {
            price_loading.store(true, Ordering::SeqCst);
            spawn_price_update_task(api_key, Arc::clone(&eth_price), Arc::clone(&price_loading));
        }

        Ok(Self {
            mode: AppMode::Normal,
            view_state: ViewState::Home,
            input: String::new(),
            cursor_position: 0,
            state,
            scroll_offset: 0,
            should_quit: false,
            eth_price,
            price_loading,
            api_key_input: String::new(),
            api_key_cursor: 0,
        })
    }

    pub async fn run(&mut self, terminal: &mut AppTerminal) -> std::io::Result<()> {
        let mut event_handler = EventHandler::new(TICK_RATE_MS);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| draw(f, self))?;

            if let Some(action) = self.poll_action(last_tick)? {
                self.execute_action(action).await;
            }

            if last_tick.elapsed() >= Duration::from_millis(TICK_RATE_MS) {
                event_handler.update_tick();
                last_tick = Instant::now();
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn poll_action(&self, last_tick: Instant) -> std::io::Result<Option<AppAction>> {
        let timeout = Duration::from_millis(TICK_RATE_MS)
            .saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)?
            && let crossterm::event::Event::Key(key) = crossterm::event::read()?
        {
            return Ok(self.dispatch_key(key));
        }

        Ok(None)
    }

    fn dispatch_key(&self, key: crossterm::event::KeyEvent) -> Option<AppAction> {
        match self.mode {
            AppMode::Normal | AppMode::Loading | AppMode::Error => handle_normal_mode(key),
            AppMode::Input => handle_input_mode(key),
            AppMode::ApiKeySetup => handle_api_key_mode(key),
        }
    }

    async fn execute_action(&mut self, action: AppAction) {
        use AppAction::*;
        
        match action {
            Quit => self.quit(),
            GoHome => self.go_home(),
            
            EnterInput => self.enter_input_mode(),
            CancelInput => self.exit_input_mode(),
            Submit => {
                if let Err(e) = self.submit_input().await {
                    self.view_state = ViewState::Error(format!("Error: {}", e));
                    self.mode = AppMode::Normal;
                }
            }
            InsertChar(c) => self.insert_char(c),
            DeleteChar => self.delete_char(),
            MoveCursorLeft => self.move_cursor_left(),
            MoveCursorRight => self.move_cursor_right(),
            MoveCursorStart => self.move_cursor_start(),
            MoveCursorEnd => self.move_cursor_end(),
            
            EnterApiKeySetup => self.enter_api_key_setup(),
            CancelApiKeySetup => self.exit_api_key_setup(),
            SubmitApiKey => {
                if let Err(e) = self.submit_api_key().await {
                    self.view_state = ViewState::Error(format!("Error: {}", e));
                    self.mode = AppMode::Normal;
                }
            }
            InsertApiKeyChar(c) => self.insert_api_key_char(c),
            DeleteApiKeyChar => self.delete_api_key_char(),
            MoveApiKeyCursorLeft => self.move_api_key_cursor_left(),
            MoveApiKeyCursorRight => self.move_api_key_cursor_right(),
            MoveApiKeyCursorStart => self.move_api_key_cursor_start(),
            MoveApiKeyCursorEnd => self.move_api_key_cursor_end(),
            
            ScrollUp => self.scroll_up(1),
            ScrollDown => self.scroll_down(1),
            ScrollUpFast => self.scroll_up(5),
            ScrollDownFast => self.scroll_down(5),
            ScrollToTop => self.reset_scroll(),
            ScrollToBottom => self.scroll_down(1000),
        }
    }

    pub fn get_eth_price(&self) -> Option<f64> {
        self.eth_price.read().ok().and_then(|p| *p)
    }
    
    pub fn is_price_loading(&self) -> bool {
        self.price_loading.load(Ordering::SeqCst)
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn enter_input_mode(&mut self) {
        self.mode = AppMode::Input;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn exit_input_mode(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn enter_api_key_setup(&mut self) {
        self.mode = AppMode::ApiKeySetup;
        self.api_key_input.clear();
        self.api_key_cursor = 0;
    }

    pub fn exit_api_key_setup(&mut self) {
        self.mode = AppMode::Normal;
        self.api_key_input.clear();
        self.api_key_cursor = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn insert_api_key_char(&mut self, c: char) {
        self.api_key_input.insert(self.api_key_cursor, c);
        self.api_key_cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn delete_api_key_char(&mut self) {
        if self.api_key_cursor > 0 {
            self.api_key_cursor -= 1;
            self.api_key_input.remove(self.api_key_cursor);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_api_key_cursor_left(&mut self) {
        if self.api_key_cursor > 0 {
            self.api_key_cursor -= 1;
        }
    }

    pub fn move_api_key_cursor_right(&mut self) {
        if self.api_key_cursor < self.api_key_input.len() {
            self.api_key_cursor += 1;
        }
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.len();
    }

    pub fn move_api_key_cursor_start(&mut self) {
        self.api_key_cursor = 0;
    }

    pub fn move_api_key_cursor_end(&mut self) {
        self.api_key_cursor = self.api_key_input.len();
    }

    pub async fn submit_api_key(&mut self) -> Result<()> {
        let api_key = self.api_key_input.trim().to_string();
        
        if api_key.is_empty() {
            self.view_state = ViewState::Error("API key cannot be empty".to_string());
            self.mode = AppMode::Normal;
            return Ok(());
        }

        self.state.config.set_api_key(api_key)?;
        
        let api_key_clone = self.state.config.etherscan_api_key.clone().unwrap();
        self.price_loading.store(true, Ordering::SeqCst);
        spawn_price_update_task(api_key_clone, Arc::clone(&self.eth_price), Arc::clone(&self.price_loading));

        self.view_state = ViewState::Home;
        self.mode = AppMode::Normal;
        self.api_key_input.clear();
        self.api_key_cursor = 0;
        
        Ok(())
    }

    pub async fn submit_input(&mut self) -> Result<()> {
        let trimmed = self.input.trim();
        
        if trimmed.is_empty() {
            return Ok(());
        }

        self.mode = AppMode::Loading;

        let view_state = match InputParser::parse(trimmed) {
            Ok(input_type) => handle_route_result(CommandRouter::route_tui(&self.state, input_type).await),
            Err(e) => ViewState::Error(format!("Invalid input: {}", e)),
        };

        self.view_state = view_state;
        self.mode = AppMode::Normal;
        self.input.clear();
        self.cursor_position = 0;
        
        Ok(())
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: u16) {
        self.scroll_offset += amount;
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn go_home(&mut self) {
        self.view_state = ViewState::Home;
        self.scroll_offset = 0;
    }
}

fn spawn_price_update_task(api_key: String, price_clone: Arc<RwLock<PriceState>>, loading_clone: Arc<AtomicBool>) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(10));
        update_price(&api_key, &price_clone, &loading_clone).await;
        
        loop {
            ticker.tick().await;
            update_price(&api_key, &price_clone, &loading_clone).await;
        }
    });
}

async fn update_price(api_key: &str, price_lock: &Arc<RwLock<PriceState>>, loading_flag: &Arc<AtomicBool>) {
    if let Ok((price, _)) = crate::api::get_eth_price(api_key).await {
        if let Ok(mut p) = price_lock.write() {
            *p = Some(price);
        }
        loading_flag.store(false, Ordering::SeqCst);
    }
}

fn handle_route_result(result: Result<ViewState>) -> ViewState {
    match result {
        Ok(view_state) => view_state,
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("API key not configured") {
                ViewState::ApiKeyRequired(
                    "Etherscan API key required for address/contract queries. Press 's' to set up your API key.".to_string()
                )
            } else {
                ViewState::Error(format!("Error: {}", e))
            }
        }
    }
}
