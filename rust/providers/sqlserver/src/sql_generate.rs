use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "[";
pub const QUOTE_RIGHT: &str = "]";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "NVARCHAR(MAX)";

pub static TYPES: LazyLock<TypeMap> = LazyLock::new(|| {
    TypeMap::new(HashMap::from([
        ("string".into(), "NVARCHAR(MAX)".into()),
        ("character".into(), "NCHAR(1)".into()),
        ("integer".into(), "INT".into()),
        ("unsignedinteger".into(), "BIGINT".into()),
        ("smallinteger".into(), "SMALLINT".into()),
        ("unsignedsmallinteger".into(), "INT".into()),
        ("biginteger".into(), "BIGINT".into()),
        ("unsignedbiginteger".into(), "DECIMAL(20)".into()),
        ("float".into(), "FLOAT".into()),
        ("decimal".into(), "DECIMAL".into()),
        ("boolean".into(), "BIT".into()),
        ("datetime".into(), "DATETIME2".into()),
        ("binary".into(), "VARBINARY(MAX)".into()),
        ("uuid".into(), "UNIQUEIDENTIFIER".into()),
        ("reference".into(), "INT".into()),
    ]))
});

pub fn convert_type(field: &SpecField) -> Result<String> {
    TYPES.convert_field(field)
}

pub fn convert_default(spec_type: &str, text: &str) -> Result<Option<String>> {
    DEFAULTS.convert(spec_type, text)
}

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "NEWID()".into(),
        now: || "GETDATE()".into(),
        utc_now: || "GETUTCDATE()".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("0x{arg}"),
        hex_reverse: Regex::new(r"^0x(.*)$").expect("hex"),
        quote: None,
        quote_reverse: None,
    })
});

const KEYWORDS: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AT", "BEGIN", "BETWEEN", "BIGINT", "BIT", "BY", "CASCADE",
    "CAST", "CHAR", "CONSTRAINT", "CREATE", "DATETIME", "DATETIME2", "DECIMAL", "DECLARE",
    "DEFAULT", "DELETE", "DROP", "EACH", "ELSE", "END", "EXECUTE", "EXISTS", "FALSE", "FLOAT",
    "FOR", "FOREIGN", "FROM", "GET", "GETDATE", "GETUTCDATE", "GO", "IDENTITY", "IDENTITY_INSERT",
    "IF", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "MAX",
    "NCHAR", "NEWID", "NOCOUNT", "NOT", "NULL", "NVARCHAR", "OFF", "ON", "OR", "ORDER",
    "PRECISION", "PRIMARY", "PROCEDURE", "REFERENCES", "RETURN", "ROW", "ROWCOUNT",
    "SCOPE_IDENTITY", "SELECT", "SET", "SMALLINT", "TABLE", "TEXT", "THEN", "TIME", "TIMESTAMP",
    "TINYINT", "TRIGGER", "TRUE", "UNIQUE", "UNIQUEIDENTIFIER", "UPDATE", "VALUES", "VARBINARY",
    "VARCHAR", "WHEN", "WHERE",
];

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"('(?:''|[^'])*')|(\[[^\]]*\])|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?P<word>[A-Za-z_][A-Za-z0-9_]*)"#)
        .expect("token")
});

pub fn apply_keywords(sql: &str, keyword: &dyn Fn(&str) -> String) -> String {
    TOKEN
        .replace_all(sql, |caps: &regex::Captures| {
            if let Some(ident) = caps.name("word") {
                if KEYWORDS.iter().any(|word| word.eq_ignore_ascii_case(ident.as_str())) {
                    return keyword(ident.as_str());
                }
            }
            caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default()
        })
        .into_owned()
}
