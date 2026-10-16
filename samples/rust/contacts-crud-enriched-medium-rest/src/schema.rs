use std::fs;
use std::path::{Path, PathBuf};

pub const GENERATED_SQL: &str = concat!(
    include_str!("../generated/sql/sqlite/migrations/0001_up.sql"),
    "\n",
    include_str!("../generated/sql/sqlite/migrations/0002_up.sql"),
    "\n",
    include_str!("../generated/sql/sqlite/migrations/0003_up.sql"),
);

fn is_up(name: &str) -> bool {
    name.len() == 11
        && name.ends_with("_up.sql")
        && name.as_bytes()[..4].iter().all(|byte| byte.is_ascii_digit())
        && !name.ends_with("_stored_procedures_up.sql")
}

fn is_migrations(dir: &Path) -> bool {
    dir.is_dir()
        && fs::read_dir(dir).ok().is_some_and(|entries| {
            entries.flatten().any(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                is_up(&name) || name == "0001_initial_up.sql"
            })
        })
}

pub fn sql_path(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let mut dir = start.as_ref().to_path_buf();
    loop {
        let direct = dir
            .join("generated")
            .join("sql")
            .join("sqlite")
            .join("migrations");
        if is_migrations(&direct) {
            return Ok(direct);
        }
        let nested = dir
            .join("samples")
            .join("rust")
            .join("contacts-crud-enriched-medium-rest")
            .join("generated")
            .join("sql")
            .join("sqlite")
            .join("migrations");
        if is_migrations(&nested) {
            return Ok(nested);
        }
        if !dir.pop() {
            return Err("generated sql".into());
        }
    }
}

pub fn up_sql_from(path: impl AsRef<Path>) -> Result<String, String> {
    let path = path.as_ref();
    if path.is_file() {
        return fs::read_to_string(path).map_err(|_| "generated sql".into());
    }
    let mut names: Vec<_> = fs::read_dir(path)
        .map_err(|_| "generated sql".to_string())?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| is_up(name))
        .collect();
    names.sort();
    if !names.is_empty() {
        return Ok(names
            .into_iter()
            .map(|name| fs::read_to_string(path.join(name)).expect("up"))
            .collect::<Vec<_>>()
            .join("\n"));
    }
    let initial = path.join("0001_initial_up.sql");
    fs::read_to_string(initial).map_err(|_| "generated sql".into())
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
    let mut dollar = false;
    while i < chars.len() {
        if chars[i] == '$' && chars.get(i + 1) == Some(&'$') {
            dollar = !dollar;
            i += 2;
            continue;
        }
        if !dollar {
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
        }
        i += 1;
    }
    out
}

pub fn with_seed_version(sql: &str) -> String {
    if !sql.starts_with("INSERT INTO") || sql.contains("\"version\"") || sql.contains("`version`")
    {
        return sql.to_string();
    }
    match sql.find(") VALUES (") {
        None => sql.to_string(),
        Some(values) => {
            let quote = if sql.contains('`') { '`' } else { '"' };
            format!(
                "{}, {quote}version{quote}) VALUES ({}, X'00');",
                &sql[..values],
                &sql[values + ") VALUES (".len()..sql.len() - 2]
            )
        }
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
