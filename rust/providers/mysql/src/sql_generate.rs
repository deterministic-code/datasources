use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "`";
pub const QUOTE_RIGHT: &str = "`";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "JSON";

pub fn procedure_specs(
    entity_name: &str,
    by_groups: &[Vec<String>],
    routine_name: &dyn Fn(&str) -> String,
    plural_stem: &dyn Fn(&str) -> String,
) -> Vec<(String, String, Vec<String>)> {
    crate_specs(entity_name, by_groups, routine_name, plural_stem)
}

fn crate_specs(
    entity_name: &str,
    by_groups: &[Vec<String>],
    routine_name: &dyn Fn(&str) -> String,
    plural_stem: &dyn Fn(&str) -> String,
) -> Vec<(String, String, Vec<String>)> {
    let plural = plural_stem(entity_name);
    let suffix = |props: &[String]| {
        if props.is_empty() {
            String::new()
        } else {
            format!("_by_{}", props.join("_"))
        }
    };
    let mut out = vec![
        ("create".into(), routine_name(&format!("create_{plural}")), vec![]),
        ("read".into(), routine_name(&format!("read_{plural}")), vec![]),
    ];
    for props in by_groups {
        let by = suffix(props);
        out.push(("read".into(), routine_name(&format!("read_{plural}{by}")), props.clone()));
        out.push(("update".into(), routine_name(&format!("update_{plural}{by}")), props.clone()));
        out.push(("delete".into(), routine_name(&format!("delete_{plural}{by}")), props.clone()));
    }
    out.push(("bulkCreate".into(), routine_name(&format!("bulk_create_{plural}")), vec![]));
    out.push(("bulkUpdate".into(), routine_name(&format!("bulk_update_{plural}")), vec![]));
    out.push(("bulkDelete".into(), routine_name(&format!("bulk_delete_{plural}")), vec![]));
    out
}

pub static TYPES: LazyLock<TypeMap> = LazyLock::new(|| {
    TypeMap::new(HashMap::from([
        ("string".into(), "TEXT".into()),
        ("character".into(), "CHAR(1)".into()),
        ("integer".into(), "INT".into()),
        ("unsignedinteger".into(), "INT UNSIGNED".into()),
        ("smallinteger".into(), "SMALLINT".into()),
        ("unsignedsmallinteger".into(), "SMALLINT UNSIGNED".into()),
        ("biginteger".into(), "BIGINT".into()),
        ("unsignedbiginteger".into(), "BIGINT UNSIGNED".into()),
        ("float".into(), "DOUBLE".into()),
        ("decimal".into(), "DECIMAL(10, 2)".into()),
        ("boolean".into(), "TINYINT(1)".into()),
        ("datetime".into(), "DATETIME(3)".into()),
        ("binary".into(), "BLOB".into()),
        ("uuid".into(), "CHAR(36)".into()),
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
        new_id: || "(UUID())".into(),
        now: || "(NOW(3))".into(),
        utc_now: || "(UTC_TIMESTAMP(3))".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("(X'{arg}')"),
        hex_reverse: Regex::new(r"^(?:\(X'(.*)'\)|X'(.*)')$").expect("hex"),
        quote: Some(mysql_quote),
        quote_reverse: Some(Regex::new(r"^(?:\('(.*)'\)|'(.*)')$").expect("quote")),
    })
});

fn mysql_quote(arg: &str) -> String {
    format!("({})", DefaultValueConverter::sql_string_literal(arg))
}

const KEYWORDS: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AUTO_INCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT",
    "BLOB", "BOOLEAN", "BY", "CASCADE", "CAST", "CHAR", "CONSTRAINT", "CREATE", "DATETIME",
    "DECIMAL", "DECLARE", "DEFAULT", "DELETE", "DOUBLE", "DROP", "EACH", "ELSE", "END",
    "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "GENERATED", "IF", "IN", "INDEX",
    "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "LAST_INSERT_ID", "LONGBLOB",
    "LONGTEXT", "NOT", "NOW", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "PROCEDURE",
    "REFERENCES", "REPLACE", "RETURN", "ROW", "ROW_COUNT", "SELECT", "SET", "SMALLINT", "TABLE",
    "TEXT", "THEN", "TIMESTAMP", "TINYINT", "TRIGGER", "TRUE", "UNIQUE", "UNSIGNED", "UPDATE",
    "UTC_TIMESTAMP", "VALUES", "VARBINARY", "VARCHAR", "WHEN", "WHERE",
];

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"('(?:''|[^'])*')|(`(?:``|[^`])*`)|("(?:""|[^"])*")|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?P<word>[A-Za-z_][A-Za-z0-9_]*)"#)
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
