pub fn trim_chars(input: &str) -> String {
    let trimmed: String = input.chars().take(256).collect();

    if trimmed.len() < input.len() {
        format!("{}...", trimmed)
    } else {
        trimmed
    }
}
