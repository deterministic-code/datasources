use crate::default_value_converter::DefaultValueConverter;
use crate::default_value_options::DefaultValueOptions;
use crate::type_map::TypeMap;
use datasources::{Result, SpecField};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "\"";
pub const QUOTE_RIGHT: &str = "\"";
pub const PROCEDURE_PAYLOAD_TYPE: &str = "jsonb";

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
        ("integer".into(), "INTEGER".into()),
        ("unsignedinteger".into(), "BIGINT".into()),
        ("smallinteger".into(), "SMALLINT".into()),
        ("unsignedsmallinteger".into(), "INTEGER".into()),
        ("biginteger".into(), "BIGINT".into()),
        ("unsignedbiginteger".into(), "NUMERIC(20)".into()),
        ("float".into(), "DOUBLE PRECISION".into()),
        ("decimal".into(), "NUMERIC".into()),
        ("boolean".into(), "BOOLEAN".into()),
        ("datetime".into(), "TIMESTAMPTZ".into()),
        ("binary".into(), "BYTEA".into()),
        ("uuid".into(), "UUID".into()),
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
        new_id: || "gen_random_uuid()".into(),
        now: || "LOCALTIMESTAMP".into(),
        utc_now: || "(NOW() AT TIME ZONE 'UTC')".into(),
        boolean_on: "TRUE".into(),
        boolean_off: "FALSE".into(),
        hex: |arg| format!("'\\x{arg}'"),
        hex_reverse: Regex::new(r"^'\\x(.*)'$").expect("hex"),
        quote: None,
        quote_reverse: None,
    })
});

const KEYWORDS: &[&str] = &[
    "AFTER", "ALTER", "AND", "AS", "AT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT", "BIGSERIAL",
    "BOOLEAN", "BY", "BYTEA", "CASCADE", "CAST", "CHAR", "CONSTRAINT", "CREATE", "DECIMAL",
    "DECLARE", "DEFAULT", "DELETE", "DIAGNOSTICS", "DOUBLE", "DROP", "EACH", "ELSE", "END",
    "EXCEPTION", "EXECUTE", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "FUNCTION",
    "GENERATED", "GET", "IF", "IMMEDIATE", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER",
    "INTO", "JOIN", "KEY", "LANGUAGE", "NEW", "NOT", "NOW", "NULL", "NUMERIC", "OLD", "ON", "OR",
    "ORDER", "OTHERS", "PRECISION", "PRIMARY", "PROCEDURE", "RAISE", "REAL", "REFERENCES",
    "REPLACE", "RETURN", "RETURNING", "RETURNS", "ROW", "ROW_COUNT", "SELECT", "SERIAL", "SET",
    "SETOF", "SMALLINT", "SQLCODE", "TABLE", "TEXT", "THEN", "TIME", "TIMESTAMP", "TIMESTAMPTZ",
    "TRIGGER", "TRUE", "UNIQUE", "UPDATE", "UUID", "VALUES", "VARCHAR", "WHEN", "WHERE", "ZONE",
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
