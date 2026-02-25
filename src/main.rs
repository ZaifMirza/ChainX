//! ChainX - Terminal Blockchain Explorer

use chainx::tui::{app::App, setup, restore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup()?;
    let mut app = App::new().await?;
    let result = app.run(&mut terminal).await;
    restore()?;
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    
    Ok(())
}
