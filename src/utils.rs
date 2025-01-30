pub fn trim_chars(input: &str) -> String {
    let trimmed: String = input.chars().take(256).collect();

    if trimmed.len() < input.len() {
        format!("{}...", trimmed)
    } else {
        trimmed
    }
}

pub fn remove_html_tags(input: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for c in input.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }

    result
}
