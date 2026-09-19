pub fn sqlite_filename(url: &str) -> Option<String> {
    let rest = url
        .get(7..)
        .filter(|_| url.len() >= 7 && url[..7].eq_ignore_ascii_case("sqlite:"))?;
    let path = rest
        .strip_prefix("///")
        .or_else(|| rest.strip_prefix("//"))
        .unwrap_or(rest);
    Some(
        if path.is_empty() || path == ":memory" || path == ":memory:" {
            ":memory:".into()
        } else {
            path.into()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::sqlite_filename;

    #[test]
    fn parses_sqlalchemy_sqlite_urls() {
        assert_eq!(
            sqlite_filename("sqlite:///:memory:").as_deref(),
            Some(":memory:")
        );
        assert_eq!(sqlite_filename("sqlite://").as_deref(), Some(":memory:"));
        assert_eq!(
            sqlite_filename("sqlite://:memory:").as_deref(),
            Some(":memory:")
        );
        assert_eq!(
            sqlite_filename("sqlite:///app.db").as_deref(),
            Some("app.db")
        );
        assert_eq!(
            sqlite_filename("sqlite:////tmp/app.db").as_deref(),
            Some("/tmp/app.db")
        );
        assert_eq!(sqlite_filename("postgres://localhost/db"), None);
    }
}
