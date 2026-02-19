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

/// Format a u64 timestamp (Unix seconds) to a readable string
pub fn format_timestamp_u64(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Format age from a Unix timestamp
pub fn format_age(timestamp: u64) -> String {
    let now = chrono::Utc::now().timestamp() as u64;
    let diff = now.saturating_sub(timestamp);

    format_duration(diff)
}

#[allow(dead_code)]
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 24 {
        let days = hours / 24;
        let remaining_hours = hours % 24;
        format!("{}d {}h ago", days, remaining_hours)
    } else if hours > 0 {
        format!("{}h {}m ago", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s ago", minutes, secs)
    } else {
        format!("{}s ago", secs)
    }
}
