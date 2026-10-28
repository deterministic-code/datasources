use datasources::{DatasourceError, Result};
use std::collections::HashMap;

pub const DEFAULT: &str = "GO";

pub fn of(settings: &HashMap<String, String>) -> Result<String> {
    match settings.get("datasource.batch_separator") {
        Some(value) if value.contains('\n') || value.contains('\r') => {
            Err(DatasourceError::message("datasource.batch_separator must be a single line"))
        }
        Some(value) => Ok(value.clone()),
        None => Ok(DEFAULT.into()),
    }
}

pub fn join(parts: impl IntoIterator<Item = impl AsRef<str>>, separator: &str) -> String {
    let chunks: Vec<String> = parts
        .into_iter()
        .map(|part| part.as_ref().to_string())
        .filter(|part| !part.is_empty())
        .collect();
    if chunks.is_empty() {
        return String::new();
    }
    let token = if separator.trim().is_empty() {
        "\n".into()
    } else {
        format!("\n{separator}\n")
    };
    chunks.join(&token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_with_a_whole_line_separator() {
        assert_eq!(
            join(["CREATE TABLE t (id INT);", "CREATE TRIGGER tr"], "GO"),
            "CREATE TABLE t (id INT);\nGO\nCREATE TRIGGER tr"
        );
    }

    #[test]
    fn drops_empty_chunks() {
        assert_eq!(join(["a", "", "  ", "b"], "GO"), "a\nGO\n  \nGO\nb");
    }

    #[test]
    fn uses_a_newline_when_the_separator_is_blank() {
        assert_eq!(join(["a", "b"], ""), "a\nb");
        assert_eq!(join(["a", "b"], "   "), "a\nb");
    }
}
