use datasources::{
    DecimalRule, DefaultValueConverter, DefaultValueOptions, DeterministicTypeConverter, SpecFieldRules,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "`";
pub const QUOTE_RIGHT: &str = "`";

fn string_sized(n: i32) -> String {
    if n <= 16383 {
        format!("VARCHAR({n})")
    } else {
        "LONGTEXT".into()
    }
}

fn binary_sized(n: i32) -> String {
    if n <= 65535 {
        format!("VARBINARY({n})")
    } else {
        "LONGBLOB".into()
    }
}

pub static TYPES: LazyLock<DeterministicTypeConverter> = LazyLock::new(|| {
    DeterministicTypeConverter::new(
        HashMap::from([
            ("string".into(), "TEXT".into()),
            ("character".into(), "CHAR(1)".into()),
            ("number".into(), "INT".into()),
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
        ]),
        SpecFieldRules {
            dialect_name: Some("mysql".into()),
            string_unlimited: "TEXT".into(),
            string_sized,
            character: Some("CHAR".into()),
            decimal: Some(DecimalRule::Optional {
                type_name: "DECIMAL".into(),
                fallback: "DECIMAL(10, 2)".into(),
            }),
            binary_unlimited: Some("BLOB".into()),
            binary_sized: Some(binary_sized),
        },
    )
});

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "(UUID())".into(),
        now: || "(NOW(3))".into(),
        utc_now: || "(UTC_TIMESTAMP(3))".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("(X'{arg}')"),
        hex_reverse: Regex::new(r"^\(?X'(.*)'\)?$").expect("hex"),
        quote: Some(mysql_quote),
        quote_reverse: Some(Regex::new(r"^\(?'(.*)'\)?$").expect("quote")),
    })
});

fn mysql_quote(arg: &str) -> String {
    format!("({})", DefaultValueConverter::sql_string_literal(arg))
}
