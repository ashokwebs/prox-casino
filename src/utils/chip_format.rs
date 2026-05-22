pub fn format_chips(amount: i64) -> String {
    let abs = amount.unsigned_abs();
    let sign = if amount < 0 { "-" } else { "" };

    if abs >= 1_000_000_000 {
        format!("{}{:.2}B", sign, abs as f64 / 1_000_000_000.0)
    } else if abs >= 1_000_000 {
        format!("{}{:.2}M", sign, abs as f64 / 1_000_000.0)
    } else if abs >= 1_000 {
        format!("{}{:.2}K", sign, abs as f64 / 1_000.0)
    } else {
        format!("{}{}", sign, abs)
    }
}

pub fn format_chips_long(amount: i64) -> String {
    let abs = amount.unsigned_abs();
    let sign = if amount < 0 { "-" } else { "" };
    let s = abs.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(ch);
    }
    format!("{}{}", sign, result)
}
