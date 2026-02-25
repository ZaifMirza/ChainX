// Status display utilities

#[allow(dead_code)]
pub fn get_status_display(status: Option<&str>) -> &'static str {
    match status {
        Some("0x1") => "✅ SUCCESS",
        Some("0x0") => "❌ FAILED",
        None => "⏳ PENDING",
        _ => "❓ UNKNOWN",
    }
}
