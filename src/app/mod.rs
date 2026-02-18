// Application runner

pub mod config;
pub mod input;

pub use config::{AppConfig, AppState};
pub use input::{InputParser, read_user_input};

use crate::commands::CommandRouter;
use crate::formatting::print_header;
use crate::error::Result;

pub struct Application;

impl Application {
    pub async fn run() -> Result<()> {
        // Load configuration
        let app_config = AppConfig::load()?;
        
        // Initialize state
        let state = AppState::new(app_config);
        
        // Main loop - keep prompting for input
        loop {
            // Print header
            print_header("ETHEREUM BLOCKCHAIN EXPLORER");
            println!();
            
            // Read and parse input
            let input = read_user_input()?;
            
            // Check for exit/quit command
            let trimmed = input.trim().to_lowercase();
            if trimmed == "quit" || trimmed == "exit" || trimmed == "q" || trimmed == "x" {
                println!("\n👋 Goodbye! Thanks for using ChainX!\n");
                break;
            }
            
            let input_type = InputParser::parse(&input)?;
            
            // Route to appropriate command
            CommandRouter::route(&state, input_type).await?;
            
            // Add a separator between queries
            println!("\n{}\n", "-".repeat(50));
        }
        
        Ok(())
    }
}
