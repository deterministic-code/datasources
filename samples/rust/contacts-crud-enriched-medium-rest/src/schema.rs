use std::path::{Path, PathBuf};

pub const GENERATED_SQL: &str =
    include_str!("../generated/sql/sqlite/migrations/0001_initial_up.sql");

pub fn sql_path(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let mut dir = start.as_ref().to_path_buf();
    loop {
        let direct = dir
            .join("generated")
            .join("sql")
            .join("sqlite")
            .join("migrations")
            .join("0001_initial_up.sql");
        if direct.is_file() {
            return Ok(direct);
        }
        let nested = dir
            .join("samples")
            .join("rust")
            .join("contacts-crud-enriched-medium-rest")
            .join("generated")
            .join("sql")
            .join("sqlite")
            .join("migrations")
            .join("0001_initial_up.sql");
        if nested.is_file() {
            return Ok(nested);
        }
        if !dir.pop() {
            return Err("generated sql".into());
        }
    }
}

pub fn statements(sql: &str) -> Vec<String> {
    let cleaned = sql
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    let chars: Vec<char> = cleaned.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if word(&cleaned, byte_index(&cleaned, i), "BEGIN") {
            depth += 1;
        }
        if chars[i] == ';' {
            if ends_with_word(&cleaned, start, byte_index(&cleaned, i), "END") && depth > 0 {
                depth -= 1;
            }
            if depth == 0 {
                let text = with_seed_version(cleaned[start..=byte_index(&cleaned, i)].trim());
                if !text.is_empty() {
                    out.push(text);
                }
                start = byte_index(&cleaned, i) + 1;
            }
        }
        i += 1;
    }
    out
}

pub fn with_seed_version(sql: &str) -> String {
    if !sql.starts_with("INSERT INTO") || sql.contains("\"version\"") {
        return sql.to_string();
    }
    match sql.find(") VALUES (") {
        None => sql.to_string(),
        Some(values) => format!(
            "{}, \"version\") VALUES ({}, X'00');",
            &sql[..values],
            &sql[values + ") VALUES (".len()..sql.len() - 2]
        ),
    }
}

fn word(sql: &str, i: usize, token: &str) -> bool {
    sql.get(i..).is_some_and(|rest| rest.starts_with(token))
        && (i == 0 || !sql.as_bytes()[i - 1].is_ascii_alphanumeric() && sql.as_bytes()[i - 1] != b'_')
        && {
            let end = i + token.len();
            end == sql.len()
                || (!sql.as_bytes()[end].is_ascii_alphanumeric() && sql.as_bytes()[end] != b'_')
        }
}

fn ends_with_word(sql: &str, start: usize, semicolon: usize, token: &str) -> bool {
    let prefix = &sql[start..semicolon];
    let trimmed = prefix.trim_end();
    trimmed.ends_with(token)
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

pub fn load_statements() -> Vec<String> {
    statements(GENERATED_SQL)
}
