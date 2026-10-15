use datasources::{
    DecimalRule, DefaultValueConverter, DefaultValueOptions, DeterministicTypeConverter, SpecFieldRules,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub const QUOTE_LEFT: &str = "[";
pub const QUOTE_RIGHT: &str = "]";

fn string_sized(n: i32) -> String {
    format!("NVARCHAR({n})")
}

fn binary_sized(n: i32) -> String {
    format!("VARBINARY({n})")
}

pub static TYPES: LazyLock<DeterministicTypeConverter> = LazyLock::new(|| {
    DeterministicTypeConverter::new(
        HashMap::from([
            ("string".into(), "NVARCHAR(MAX)".into()),
            ("character".into(), "NCHAR(1)".into()),
            ("number".into(), "INT".into()),
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
        ]),
        SpecFieldRules {
            dialect_name: Some("sqlserver".into()),
            string_unlimited: "NVARCHAR(MAX)".into(),
            string_sized,
            character: Some("NCHAR".into()),
            decimal: Some(DecimalRule::Required {
                type_name: "DECIMAL".into(),
            }),
            binary_unlimited: Some("VARBINARY(MAX)".into()),
            binary_sized: Some(binary_sized),
        },
    )
});

pub static DEFAULTS: LazyLock<DefaultValueConverter> = LazyLock::new(|| {
    DefaultValueConverter::new(DefaultValueOptions {
        new_id: || "NEWID()".into(),
        now: || "GETDATE()".into(),
        utc_now: || "GETUTCDATE()".into(),
        boolean_on: "1".into(),
        boolean_off: "0".into(),
        hex: |arg| format!("0x{arg}"),
        hex_reverse: Regex::new(r"^0x(.*)$").expect("hex"),
    })
});
