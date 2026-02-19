// TUI Application state and logic

use crate::app::{AppConfig, AppState};
use crate::commands::CommandRouter;
use crate::error::Result;
use crate::models::{
    AddressDisplay, BlockDisplay, ContractDisplay, TransactionDisplay,
};
use crate::app::input::{InputParser, InputType};
use std::sync::{Arc, RwLock};
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    Loading,
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
}

pub struct App {
    pub mode: AppMode,
    pub view_state: ViewState,
    pub input: String,
    pub cursor_position: usize,
    pub state: AppState,
    pub scroll_offset: u16,
    pub should_quit: bool,
    pub eth_price: Arc<RwLock<f64>>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let app_config = AppConfig::load()?;
        let state = AppState::new(app_config);
        let eth_price = Arc::new(RwLock::new(0.0));

        // Spawn background task to update ETH price every 10 seconds
        let api_key = state.config.etherscan_api_key.clone();
        let price_clone = Arc::clone(&eth_price);
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(10));
            
            // Initial fetch
            if let Ok((price, _)) = crate::api::get_eth_price(&api_key).await {
                if let Ok(mut p) = price_clone.write() {
                    *p = price;
                }
            }
            
            // Update every 10 seconds
            loop {
                ticker.tick().await;
                if let Ok((price, _)) = crate::api::get_eth_price(&api_key).await {
                    if let Ok(mut p) = price_clone.write() {
                        *p = price;
                    }
                }
            }
        });

        Ok(Self {
            mode: AppMode::Normal,
            view_state: ViewState::Home,
            input: String::new(),
            cursor_position: 0,
            state,
            scroll_offset: 0,
            should_quit: false,
            eth_price,
        })
    }

    pub fn get_eth_price(&self) -> f64 {
        self.eth_price.read().map(|p| *p).unwrap_or(0.0)
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

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
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

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.len();
    }

    pub async fn submit_input(&mut self) -> Result<()> {
        let trimmed = self.input.trim();
        
        if trimmed.is_empty() {
            return Ok(());
        }

        self.mode = AppMode::Loading;

        match InputParser::parse(trimmed) {
            Ok(input_type) => {
                match CommandRouter::route_tui(&self.state, input_type).await {
                    Ok(result) => {
                        self.view_state = result;
                        self.scroll_offset = 0;
                    }
                    Err(e) => {
                        self.view_state = ViewState::Error(format!("Error: {}", e));
                    }
                }
            }
            Err(e) => {
                self.view_state = ViewState::Error(format!("Invalid input: {}", e));
            }
        }

        self.mode = AppMode::Normal;
        self.input.clear();
        self.cursor_position = 0;
        
        Ok(())
    }

    pub fn scroll_up(&mut self, amount: u16) {
        if self.scroll_offset >= amount {
            self.scroll_offset -= amount;
        } else {
            self.scroll_offset = 0;
        }
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
