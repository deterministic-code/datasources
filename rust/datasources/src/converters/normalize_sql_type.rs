pub fn normalize_sql_type(sql_type: &str) -> String {
    let trimmed = sql_type.trim().to_ascii_uppercase();
    if trimmed.starts_with("TINYINT(1)") {
        return "TINYINT(1)".into();
    }
    if trimmed.starts_with("CHAR(36)") {
        return "CHAR(36)".into();
    }
    match trimmed.find('(') {
        None => trimmed,
        Some(paren) => trimmed[..paren].trim().to_string(),
    }
}
