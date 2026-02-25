//! ChainX - Terminal Blockchain Explorer
//! 
//! A beautiful TUI blockchain explorer for Ethereum.

pub mod api;
pub mod app;
pub mod cache;
pub mod commands;
pub mod config;
pub mod formatting;
pub mod models;
pub mod tui;
pub mod utils;
pub mod validation;

pub use app::{ExplorerError, Result};
