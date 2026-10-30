use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "\"";
pub const QUOTE_RIGHT: &str = "\"";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "CLOB";

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
        ("string".into(), "CLOB".into()),
        ("character".into(), "CHAR(1)".into()),
        ("integer".into(), "NUMBER(10)".into()),
        ("unsignedinteger".into(), "NUMBER(10)".into()),
        ("smallinteger".into(), "NUMBER(5)".into()),
        ("unsignedsmallinteger".into(), "NUMBER(5)".into()),
        ("biginteger".into(), "NUMBER(19)".into()),
        ("unsignedbiginteger".into(), "NUMBER(20)".into()),
        ("float".into(), "BINARY_DOUBLE".into()),
        ("decimal".into(), "NUMBER".into()),
        ("boolean".into(), "NUMBER(1)".into()),
        ("datetime".into(), "TIMESTAMP".into()),
        ("binary".into(), "BLOB".into()),
        ("uuid".into(), "VARCHAR2(36)".into()),
        ("reference".into(), "NUMBER(10)".into()),
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
        new_id: || "SYS_GUID()".into(),
        now: || "LOCALTIMESTAMP".into(),
        utc_now: || "SYS_EXTRACT_UTC(SYSTIMESTAMP)".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("HEXTORAW('{arg}')"),
        hex_reverse: Regex::new(r"^HEXTORAW\('(.*)'\)$").expect("hex"),
        quote: None,
        quote_reverse: None,
    })
});

const KEYWORDS: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AT", "BEFORE", "BEGIN", "BETWEEN", "BINARY_DOUBLE", "BLOB",
    "BY", "CASCADE", "CAST", "CHAR", "CLOB", "CONSTRAINT", "CONSTRAINTS", "CREATE", "DECIMAL",
    "DECLARE", "DEFAULT", "DELETE", "DROP", "EACH", "ELSE", "END", "EXCEPTION", "EXECUTE",
    "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "FUNCTION", "IF", "IMMEDIATE", "IN",
    "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "LANGUAGE",
    "LOCALTIMESTAMP", "NCHAR", "NOT", "NULL", "NUMBER", "NUMERIC", "OLD", "ON", "OR", "ORDER",
    "OTHERS", "PRECISION", "PRIMARY", "PROCEDURE", "RAISE", "RAW", "REFERENCES", "REPLACE",
    "RETURN", "RETURNING", "ROW", "SELECT", "SET", "SMALLINT", "SQLCODE", "SYS_EXTRACT_UTC",
    "SYS_GUID", "SYSTIMESTAMP", "TABLE", "THEN", "TIMESTAMP", "TRIGGER", "TRUE", "UNIQUE",
    "UPDATE", "VALUES", "VARCHAR", "VARCHAR2", "WHEN", "WHERE", "ZONE",
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
