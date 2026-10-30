use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "\"";
pub const QUOTE_RIGHT: &str = "\"";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "JSON";

pub fn procedure_specs(
    entity_name: &str,
    by_groups: &[Vec<String>],
    routine_name: &dyn Fn(&str) -> String,
    plural_stem: &dyn Fn(&str) -> String,
) -> Vec<(String, String, Vec<String>)> {
    specs(entity_name, by_groups, routine_name, plural_stem)
}

fn specs(
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
        ("character".into(), "TEXT".into()),
        ("integer".into(), "INTEGER".into()),
        ("unsignedinteger".into(), "INTEGER".into()),
        ("smallinteger".into(), "INTEGER".into()),
        ("unsignedsmallinteger".into(), "INTEGER".into()),
        ("biginteger".into(), "INTEGER".into()),
        ("unsignedbiginteger".into(), "INTEGER".into()),
        ("float".into(), "REAL".into()),
        ("decimal".into(), "NUMERIC".into()),
        ("boolean".into(), "BOOLEAN".into()),
        ("datetime".into(), "TEXT".into()),
        ("binary".into(), "BLOB".into()),
        ("uuid".into(), "TEXT".into()),
        ("reference".into(), "INTEGER".into()),
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
        new_id: || "(lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6))))".into(),
        now: || "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))".into(),
        utc_now: || "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("X'{arg}'"),
        hex_reverse: Regex::new(r"^X'(.*)'$").expect("hex"),
        quote: None,
        quote_reverse: None,
    })
});

const KEYWORDS: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AT", "AUTOINCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BLOB",
    "BOOLEAN", "BY", "CASCADE", "CAST", "CONSTRAINT", "CREATE", "DECIMAL", "DEFAULT", "DELETE",
    "DROP", "EACH", "ELSE", "END", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM",
    "GENERATED", "IF", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY",
    "NOT", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "REAL", "REFERENCES", "RETURN",
    "RETURNING", "ROW", "SELECT", "SET", "TABLE", "TEXT", "THEN", "TRIGGER", "TRUE", "UNIQUE",
    "UPDATE", "VALUES", "VARCHAR", "WHEN", "WHERE",
];

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"('(?:''|[^'])*')|("(?:""|[^"])*")|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?P<word>[A-Za-z_][A-Za-z0-9_]*)"#)
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
