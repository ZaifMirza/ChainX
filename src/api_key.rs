// API Key configuration storage
// Stores the Etherscan API key in a config file for persistence

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const CONFIG_DIR: &str = "chainx";
const CONFIG_FILE: &str = "config.toml";

/// Get the path to the config file
fn get_config_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;
    
    let chainx_dir = config_dir.join(CONFIG_DIR);
    
    // Create directory if it doesn't exist
    if !chainx_dir.exists() {
        fs::create_dir_all(&chainx_dir)?;
    }
    
    Ok(chainx_dir.join(CONFIG_FILE))
}

/// Load the API key from the config file
pub fn load_api_key() -> io::Result<Option<String>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(None);
    }
    
    let contents = fs::read_to_string(&config_path)?;
    
    // Parse simple key=value format
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("etherscan_api_key") {
            if let Some(eq_pos) = line.find('=') {
                let value = line[eq_pos + 1..].trim();
                // Remove quotes if present
                let value = value.trim_matches('"').trim();
                if !value.is_empty() {
                    return Ok(Some(value.to_string()));
                }
            }
        }
    }
    
    Ok(None)
}

/// Save the API key to the config file
pub fn save_api_key(api_key: &str) -> io::Result<()> {
    let config_path = get_config_path()?;
    
    let content = format!("etherscan_api_key = \"{}\"\n", api_key);
    
    fs::write(&config_path, content)?;
    
    Ok(())
}

/// Check if API key is configured
pub fn has_api_key() -> bool {
    load_api_key().ok().flatten().is_some()
}

/// Prompt user for API key in terminal (for CLI setup)
#[allow(dead_code)]
pub fn prompt_api_key() -> io::Result<String> {
    print!("Enter your Etherscan API key: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let api_key = input.trim().to_string();
    
    if api_key.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "API key cannot be empty"));
    }
    
    Ok(api_key)
}
