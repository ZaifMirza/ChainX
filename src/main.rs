// Main entry point - minimal orchestration only

mod api;
mod app;
mod cache;
mod commands;
mod config;
mod error;
mod formatting;
mod models;
mod utils;
mod validation;

use app::Application;

#[tokio::main]
async fn main() {
    if let Err(e) = Application::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
