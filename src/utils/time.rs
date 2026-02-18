// Time formatting utilities

use crate::utils::hex::parse_hex;
use chrono;

pub fn format_timestamp(timestamp: Option<&str>) -> String {
    if let Some(ts) = timestamp {
        let ts_val = parse_hex(ts);
        chrono::DateTime::from_timestamp(ts_val as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Pending".to_string()
    }
}

#[allow(dead_code)]
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
