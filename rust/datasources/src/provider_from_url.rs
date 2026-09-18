pub fn provider_from_url(url: &str) -> Option<String> {
    let scheme = url.split_once(':')?.0.to_ascii_lowercase();
    if scheme.is_empty() {
        return None;
    }
    Some(
        match scheme.as_str() {
            "postgresql" => "postgres",
            "mssql" => "sqlserver",
            "sqlite3" => "sqlite",
            other => other,
        }
        .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::provider_from_url;

    #[test]
    fn reads_scheme() {
        assert_eq!(provider_from_url("sqlite:///:memory:").as_deref(), Some("sqlite"));
        assert_eq!(
            provider_from_url("postgres://ds:ds@127.0.0.1:5432/datasources").as_deref(),
            Some("postgres")
        );
        assert_eq!(provider_from_url("postgresql://localhost/db").as_deref(), Some("postgres"));
        assert_eq!(provider_from_url("mysql://localhost/db").as_deref(), Some("mysql"));
        assert_eq!(provider_from_url(""), None);
    }
}
