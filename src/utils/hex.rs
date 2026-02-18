// Hex parsing utilities

pub fn parse_hex(s: &str) -> u64 {
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

#[allow(dead_code)]
pub fn parse_hex_u128(s: &str) -> u128 {
    if s.starts_with("0x") {
        u128::from_str_radix(&s[2..], 16).unwrap_or(0)
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
    if !s.starts_with("0x") {
        return false;
    }
    s[2..].chars().all(|c| c.is_ascii_hexdigit())
}
