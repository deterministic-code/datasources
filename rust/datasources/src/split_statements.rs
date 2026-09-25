pub fn split_statements(sql: &str) -> Vec<String> {
    let chars: Vec<char> = sql.chars().collect();
    let mut parts = Vec::new();
    let mut start = 0;
    let mut quote: Option<char> = None;
    let mut line = false;
    let mut block = false;
    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        let next = chars.get(i + 1).copied();
        if line {
            if char == '\n' {
                line = false;
            }
            i += 1;
            continue;
        }
        if block {
            if char == '*' && next == Some('/') {
                block = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if let Some(mark) = quote {
            if char == mark {
                if next == Some(mark) {
                    i += 2;
                    continue;
                }
                quote = None;
            }
            i += 1;
            continue;
        }
        if char == '-' && next == Some('-') {
            line = true;
            i += 2;
            continue;
        }
        if char == '/' && next == Some('*') {
            block = true;
            i += 2;
            continue;
        }
        if matches!(char, '\'' | '"' | '`') {
            quote = Some(char);
            i += 1;
            continue;
        }
        if char == ';' {
            let part: String = chars[start..i].iter().collect();
            let part = part.trim();
            if !part.is_empty() {
                parts.push(part.to_string());
            }
            start = i + 1;
        }
        i += 1;
    }
    let tail: String = chars[start..].iter().collect();
    let tail = tail.trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }
    parts
}
