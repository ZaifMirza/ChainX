// String manipulation utilities

pub fn truncate_and_pad(s: &str, len: usize) -> String {
    if s.len() > len {
        format!("{}...", &s[..len - 3])
    } else {
        s.to_string()
    }
}

pub fn truncate_for_display(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;

    for ch in s.chars() {
        let char_width = if ch.is_ascii() { 1 } else { 2 };
        if current_width + char_width > max_width {
            if current_width + 3 <= max_width {
                return format!("{}...", result);
            } else {
                let truncate_len = result.len().saturating_sub(3);
                return format!("{}...", &result[..truncate_len]);
            }
        }
        result.push(ch);
        current_width += char_width;
    }

    result
}

pub fn center_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text.to_string();
    }
    let padding = width - text.len();
    let left = padding / 2;
    let right = padding - left;
    let mut result = String::with_capacity(width);
    result.push_str(&" ".repeat(left));
    result.push_str(text);
    result.push_str(&" ".repeat(right));
    result
}
