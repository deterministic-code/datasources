use datasources::{
    DecimalRule, DefaultValueConverter, DefaultValueOptions, DeterministicTypeConverter, SpecFieldRules,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "\"";
pub const QUOTE_RIGHT: &str = "\"";

fn string_sized(n: i32) -> String {
    format!("VARCHAR2({n})")
}

fn binary_sized(n: i32) -> String {
    format!("RAW({n})")
}

pub static TYPES: LazyLock<DeterministicTypeConverter> = LazyLock::new(|| {
    DeterministicTypeConverter::new(
        HashMap::from([
            ("string".into(), "CLOB".into()),
            ("character".into(), "CHAR(1)".into()),
            ("number".into(), "NUMBER(10)".into()),
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
        ]),
        SpecFieldRules {
            dialect_name: Some("oracle".into()),
            string_unlimited: "CLOB".into(),
            string_sized,
            character: Some("CHAR".into()),
            decimal: Some(DecimalRule::Required {
                type_name: "NUMBER".into(),
            }),
            binary_unlimited: Some("BLOB".into()),
            binary_sized: Some(binary_sized),
        },
    )
});

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "SYS_GUID()".into(),
        now: || "LOCALTIMESTAMP".into(),
        utc_now: || "SYS_EXTRACT_UTC(SYSTIMESTAMP)".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("HEXTORAW('{arg}')"),
        hex_reverse: Regex::new(r"^HEXTORAW\('(.*)'\)$").expect("hex"),
    })
});
