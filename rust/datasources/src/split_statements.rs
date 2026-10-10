pub fn split_statements(sql: &str) -> Vec<String> {
    let chars: Vec<char> = sql.chars().collect();
    let mut parts = Vec::new();
    let mut start = 0;
    let mut quote: Option<char> = None;
    let mut dollar: Option<String> = None;
    let mut line = false;
    let mut block = false;
    let mut depth = 0usize;
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
        if let Some(tag) = &dollar {
            if starts_with(&chars, i, tag) {
                i += tag.chars().count();
                dollar = None;
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
        if let Some(tag) = dollar_tag(&chars, i) {
            i += tag.chars().count();
            dollar = Some(tag);
            continue;
        }
        if matches!(char, '\'' | '"' | '`') {
            quote = Some(char);
            i += 1;
            continue;
        }
        if routine(sql, start, byte_index(sql, i)) && word(&chars, i, "BEGIN") {
            depth += 1;
            i += 5;
            continue;
        }
        if char == ';' {
            let at = byte_index(sql, i);
            if routine(sql, start, at) && depth > 0 {
                if ends_with_word(&sql[start..at], "END") {
                    depth -= 1;
                }
                if depth > 0 {
                    i += 1;
                    continue;
                }
            }
            let part = sql[start..at].trim();
            if !part.is_empty() {
                parts.push(part.to_string());
            }
            start = at + 1;
            depth = 0;
        }
        i += 1;
    }
    let tail = sql[start..].trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }
    parts
}

pub fn statement_uses_parameter(sql: &str, name: &str) -> bool {
    let bare = name.trim_start_matches(['$', '@', ':']);
    ["$", ":", "@"].iter().any(|prefix| {
        let token = format!("{prefix}{bare}");
        sql.match_indices(&token).any(|(at, _)| {
            !sql[at + token.len()..]
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphanumeric() || next == '_')
        })
    })
}

fn dollar_tag(chars: &[char], i: usize) -> Option<String> {
    if chars.get(i) != Some(&'$') {
        return None;
    }
    let mut end = i + 1;
    while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
        end += 1;
    }
    (chars.get(end) == Some(&'$')).then(|| chars[i..=end].iter().collect())
}

fn starts_with(chars: &[char], i: usize, tag: &str) -> bool {
    let needle: Vec<char> = tag.chars().collect();
    chars.get(i..i + needle.len()) == Some(needle.as_slice())
}

fn word(chars: &[char], i: usize, token: &str) -> bool {
    let needle: Vec<char> = token.chars().collect();
    if chars.get(i..i + needle.len()) != Some(needle.as_slice()) {
        return false;
    }
    let before = i == 0 || !(chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == '_');
    let after = i + needle.len() == chars.len()
        || !(chars[i + needle.len()].is_ascii_alphanumeric() || chars[i + needle.len()] == '_');
    before && after
}

fn routine(sql: &str, start: usize, i: usize) -> bool {
    let head = sql[start..i].to_ascii_uppercase();
    head.contains("CREATE PROCEDURE") || head.contains("CREATE FUNCTION")
}

fn ends_with_word(sql: &str, token: &str) -> bool {
    let trimmed = sql.trim_end();
    trimmed
        .to_ascii_uppercase()
        .ends_with(token)
        && (trimmed.len() == token.len()
            || {
                let before = trimmed.as_bytes()[trimmed.len() - token.len() - 1];
                !before.is_ascii_alphanumeric() && before != b'_'
            })
}

fn byte_index(sql: &str, char_index: usize) -> usize {
    sql.char_indices()
        .nth(char_index)
        .map(|(i, _)| i)
        .unwrap_or(sql.len())
}

#[cfg(test)]
mod tests {
    use super::{split_statements, statement_uses_parameter};

    #[test]
    fn splits_on_semicolons_outside_quotes() {
        assert_eq!(split_statements("SELECT 1"), vec!["SELECT 1"]);
        assert_eq!(
            split_statements("SELECT 1 AS id; SELECT 2 AS id"),
            vec!["SELECT 1 AS id", "SELECT 2 AS id"]
        );
        assert_eq!(
            split_statements("SELECT ';' AS id; SELECT 2 AS id"),
            vec!["SELECT ';' AS id", "SELECT 2 AS id"]
        );
    }

    #[test]
    fn keeps_procedure_bodies() {
        let sql = "CREATE PROCEDURE add_user()\nBEGIN\n  INSERT INTO t VALUES (1);\nEND";
        assert_eq!(split_statements(sql), vec![sql]);
        let dollar = "CREATE PROCEDURE bulk_create_user(payload jsonb)\nLANGUAGE plpgsql\nAS $$\nBEGIN\n  INSERT INTO t VALUES (1);\nEND;\n$$";
        assert_eq!(split_statements(dollar), vec![dollar]);
    }

    #[test]
    fn parameter_token_uses_a_word_boundary() {
        assert!(statement_uses_parameter(
            "INSERT INTO t (is_enabled) VALUES ($is_enabled__10)",
            "is_enabled__10"
        ));
        assert!(!statement_uses_parameter(
            "INSERT INTO t (is_enabled) VALUES ($is_enabled__10)",
            "is_enabled__1"
        ));
        assert!(statement_uses_parameter("$is_enabled__1", "is_enabled__1"));
    }
}
