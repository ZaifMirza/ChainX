// Time formatting utilities

use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::hex::parse_hex;

pub fn format_timestamp(timestamp: Option<&str>) -> String {
    if let Some(ts) = timestamp {
        let ts_val = parse_hex(ts);
        format_timestamp_u64(ts_val)
    } else {
        "Pending".to_string()
    }
}

/// Format a u64 timestamp (Unix seconds) to a readable string
pub fn format_timestamp_u64(timestamp: u64) -> String {
    // Convert Unix timestamp to readable format using std library
    let days_since_epoch = timestamp / 86400;
    let remaining_secs = timestamp % 86400;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    let seconds = remaining_secs % 60;
    
    // Calculate date (simplified, starts from 1970-01-01)
    let (year, month, day) = days_to_date(days_since_epoch);
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", year, month, day, hours, minutes, seconds)
}

/// Convert days since Unix epoch to (year, month, day)
fn days_to_date(days: u64) -> (u32, u32, u32) {
    let mut year = 1970u32;
    let mut remaining_days = days;
    
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    let (month, day) = days_to_month_day(remaining_days as u32, year);
    (year, month, day)
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_to_month_day(days: u32, year: u32) -> (u32, u32) {
    const DAYS_IN_MONTHS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = is_leap_year(year);
    
    let mut month = 1u32;
    let mut remaining_days = days;
    
    for (i, &days_in_month) in DAYS_IN_MONTHS.iter().enumerate() {
        let dim = if i == 1 && leap { days_in_month + 1 } else { days_in_month };
        if remaining_days < dim {
            return (month, remaining_days + 1);
        }
        remaining_days -= dim;
        month += 1;
    }
    
    (12, 31)
}

/// Get current Unix timestamp in seconds
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Format age from a Unix timestamp
pub fn format_age(timestamp: u64) -> String {
    let now = current_timestamp();
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
