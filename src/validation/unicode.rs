// Unicode spoofing detection

pub fn contains_spoofing(s: &str) -> bool {
    let suspicious_chars = [
        'ꓴ', 'ꓢ', 'ꓓ', 'ꓔ', 'ꓕ', 'ꓚ', 'ꓛ', 'ꓜ', 'ꓝ', 'ꓞ', 'ꓟ', 'ꓠ', 'ꓡ', 'ꓢ', 'ꓣ', 'ꓤ', 'ꓥ', 'ꓦ',
        'ꓧ', 'ꓨ', 'ꓩ', 'ꓪ', 'ꓫ', 'ꓬ', 'ꓭ', 'ꓮ', 'ꓯ', 'ꓰ', 'ꓱ', 'ꓲ', 'ꓳ', 'ꓴ', 'ꓵ', 'ꓶ', 'ꓷ', 'ꓸ',
        'ꓹ', 'ꓺ', 'ꓻ', 'ꓼ', 'ꓽ', '꓾', '꓿',
    ];

    s.chars().any(|ch| suspicious_chars.contains(&ch))
}

#[allow(dead_code)]
pub fn sanitize_symbol(symbol: &str) -> String {
    if contains_spoofing(symbol) {
        "⚠️ SPOOFED".to_string()
    } else {
        symbol.to_string()
    }
}
