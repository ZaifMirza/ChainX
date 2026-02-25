// Hex parsing utilities

pub fn parse_hex(s: &str) -> u64 {
    if let Some(stripped) = s.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

#[allow(dead_code)]
pub fn parse_hex_u128(s: &str) -> u128 {
    if let Some(stripped) = s.strip_prefix("0x") {
        u128::from_str_radix(stripped, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

#[allow(dead_code)]
pub fn to_hex(n: u64) -> String {
    format!("0x{:x}", n)
}

#[allow(dead_code)]
pub fn is_hex_string(s: &str) -> bool {
    let Some(stripped) = s.strip_prefix("0x") else {
        return false;
    };
    stripped.chars().all(|c| c.is_ascii_hexdigit())
}

/// Decode a hex string (with or without 0x prefix) to bytes
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    let hex_str = s.strip_prefix("0x").unwrap_or(s);
    
    if !hex_str.len().is_multiple_of(2) {
        return Err("Invalid hex string length".to_string());
    }
    
    (0..hex_str.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_str[i..i + 2], 16)
                .map_err(|_| "Invalid hex character".to_string())
        })
        .collect()
}
