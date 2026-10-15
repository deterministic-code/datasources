use regex::Regex;
use std::sync::LazyLock;

const ALL: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AT", "AUTOINCREMENT", "AUTO_INCREMENT", "BEFORE", "BEGIN",
    "BETWEEN", "BIGINT", "BIGSERIAL", "BINARY_DOUBLE", "BIT", "BLOB", "BOOLEAN", "BY", "BYTEA",
    "CASCADE", "CAST", "CHAR", "CLOB", "CONSTRAINT", "CONSTRAINTS", "CREATE", "DATETIME",
    "DATETIME2", "DECIMAL", "DECLARE", "DEFAULT", "DELETE", "DIAGNOSTICS", "DOUBLE", "DROP",
    "EACH", "ELSE", "END", "EXCEPTION", "EXECUTE", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN",
    "FROM", "FUNCTION", "GENERATED", "GET", "GETDATE", "GETUTCDATE", "GO", "IDENTITY",
    "IDENTITY_INSERT", "IF", "IMMEDIATE", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER",
    "INTO", "JOIN", "KEY", "LANGUAGE", "LAST_INSERT_ID", "LOCALTIMESTAMP", "LONGBLOB",
    "LONGTEXT", "MAX", "NCHAR", "NEW", "NEWID", "NOCOUNT", "NOT", "NOW", "NULL", "NUMBER",
    "NUMERIC", "NVARCHAR", "OFF", "OLD", "ON", "OR", "ORDER", "OTHERS", "PRECISION", "PRIMARY",
    "PROCEDURE", "RAISE", "RAW", "REAL", "REFERENCES", "REPLACE", "RETURN", "RETURNING",
    "RETURNS", "ROW", "ROWCOUNT", "ROW_COUNT", "SCOPE_IDENTITY", "SELECT", "SERIAL", "SET",
    "SETOF", "SMALLINT", "SQLCODE", "SYS_EXTRACT_UTC", "SYS_GUID", "SYSTIMESTAMP", "TABLE",
    "TEXT", "THEN", "TIME", "TIMESTAMP", "TIMESTAMPTZ", "TINYINT", "TRIGGER", "TRUE", "UNIQUE",
    "UNIQUEIDENTIFIER", "UNSIGNED", "UPDATE", "UTC_TIMESTAMP", "UUID", "VALUES", "VARBINARY",
    "VARCHAR", "VARCHAR2", "WHEN", "WHERE", "ZONE",
];

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"('(?:''|[^'])*')|("(?:""|[^"])*")|(`(?:``|[^`])*`)|(\[[^\]]*\])|(--[^\n]*)|(/\*[\s\S]*?\*/)|([A-Za-z_][A-Za-z0-9_]*)"#)
        .expect("token")
});

pub fn apply(sql: &str, keyword: impl Fn(&str) -> String) -> String {
    TOKEN
        .replace_all(sql, |caps: &regex::Captures| {
            if let Some(ident) = caps.get(7) {
                if ALL.contains(&ident.as_str().to_ascii_uppercase().as_str()) {
                    return keyword(ident.as_str());
                }
            }
            caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default()
        })
        .into_owned()
}
