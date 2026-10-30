use datasources::{
    DefaultValueConverter, DefaultValueOptions, DeterministicTypeConverter, SpecFieldRules,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "\"";
pub const QUOTE_RIGHT: &str = "\"";

fn string_sized(n: i32) -> String {
    format!("VARCHAR({n})")
}

pub static TYPES: LazyLock<DeterministicTypeConverter> = LazyLock::new(|| {
    DeterministicTypeConverter::new(
        HashMap::from([
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
        ]),
        SpecFieldRules {
            dialect_name: Some("sqlite".into()),
            string_unlimited: "TEXT".into(),
            string_sized,
            character: None,
            decimal: None,
            binary_unlimited: None,
            binary_sized: None,
        },
    )
});

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
