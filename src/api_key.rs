// API Key configuration storage

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const CONFIG_DIR: &str = "chainx";
const CONFIG_FILE: &str = "config.toml";

fn get_config_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io_error("Could not find config directory"))?;
    
    let chainx_dir = config_dir.join(CONFIG_DIR);
    
    if !chainx_dir.exists() {
        fs::create_dir_all(&chainx_dir)?;
    }
    
    Ok(chainx_dir.join(CONFIG_FILE))
}

pub fn load_api_key() -> io::Result<Option<String>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(None);
    }
    
    let contents = fs::read_to_string(&config_path)?;
    Ok(parse_api_key_from_contents(&contents))
}

pub fn save_api_key(api_key: &str) -> io::Result<()> {
    let config_path = get_config_path()?;
    let content = format!("etherscan_api_key = \"{}\"\n", api_key);
    fs::write(&config_path, content)
}

#[allow(dead_code)]
pub fn prompt_api_key() -> io::Result<String> {
    print!("Enter your Etherscan API key: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let api_key = input.trim().to_string();
    
    if api_key.is_empty() {
        return Err(io_error("API key cannot be empty"));
    }
    
    Ok(api_key)
}

fn io_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, msg)
}

fn parse_api_key_from_contents(contents: &str) -> Option<String> {
    contents
        .lines()
        .map(|line| line.trim())
        .find(|line| line.starts_with("etherscan_api_key"))
        .and_then(|line| extract_api_key_value(line))
}

fn extract_api_key_value(line: &str) -> Option<String> {
    let eq_pos = line.find('=')?;
    let value = line[eq_pos + 1..].trim();
    let value = value.trim_matches('"').trim();
    
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}
