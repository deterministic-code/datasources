use datasources::{
    DecimalRule, DefaultValueConverter, DefaultValueOptions, DeterministicTypeConverter, SpecFieldRules,
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
            ("character".into(), "CHAR(1)".into()),
            ("number".into(), "INTEGER".into()),
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
        ]),
        SpecFieldRules {
            dialect_name: Some("postgres".into()),
            string_unlimited: "TEXT".into(),
            string_sized,
            character: Some("CHAR".into()),
            decimal: Some(DecimalRule::Optional {
                type_name: "NUMERIC".into(),
                fallback: "NUMERIC".into(),
            }),
            binary_unlimited: None,
            binary_sized: None,
        },
    )
});

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "gen_random_uuid()".into(),
        now: || "LOCALTIMESTAMP".into(),
        utc_now: || "(NOW() AT TIME ZONE 'UTC')".into(),
        boolean_on: "TRUE".into(),
        boolean_off: "FALSE".into(),
        hex: |arg| format!("'\\x{arg}'"),
        hex_reverse: Regex::new(r"^'\\x(.*)'$").expect("hex"),
    })
});
