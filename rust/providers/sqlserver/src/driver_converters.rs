use crate::SqlServerDriver;
use datasources::{Driver, FieldSize, SpecField};

#[test]
fn type_converter() {
    let driver = SqlServerDriver;
    let types = driver.type_converter().expect("type map");
    assert_eq!(types.convert(&"string".into()).unwrap(), "NVARCHAR(MAX)");
    assert_eq!(types.convert(&"integer".into()).unwrap(), "INT");
    assert_eq!(types.convert(&"unsignedinteger".into()).unwrap(), "BIGINT");
    assert_eq!(types.convert(&"datetime".into()).unwrap(), "DATETIME2");
    assert_eq!(types.convert(&"uuid".into()).unwrap(), "UNIQUEIDENTIFIER");
    assert_eq!(types.convert(&"binary".into()).unwrap(), "VARBINARY(MAX)");
    assert_eq!(
        SqlServerDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..Default::default()
        })
        .unwrap(),
        "NVARCHAR(64)"
    );
    assert_eq!(
        SqlServerDriver::convert_type(&SpecField {
            type_name: "binary".into(),
            size: Some(FieldSize::Length(16)),
            ..Default::default()
        })
        .unwrap(),
        "VARBINARY(16)"
    );
    assert_eq!(types.reverse_convert(&"DATETIME2".into()).unwrap(), "datetime");
    assert!(types.convert(&"nope".into()).is_err());
}

#[test]
fn default_value_converter() {
    let defaults = SqlServerDriver.default_value_converter();
    assert_eq!(defaults.convert("datetime", "Now").unwrap().unwrap(), "GETDATE()");
    assert_eq!(defaults.convert("datetime", "UtcNow").unwrap().unwrap(), "GETUTCDATE()");
    assert_eq!(defaults.convert("uuid", "NewId").unwrap().unwrap(), "NEWID()");
    assert_eq!(defaults.convert("binary", "Hex('ab')").unwrap().unwrap(), "0xab");
    assert_eq!(defaults.convert("boolean", "true").unwrap().unwrap(), "1");
    assert_eq!(defaults.reverse_convert("uuid", "NEWID()").unwrap(), "NewId");
    assert_eq!(defaults.reverse_convert("binary", "0xab").unwrap(), "Hex('ab')");
    assert!(defaults.convert("nope", "x").is_err());
}

#[test]
fn empty_row_type_converters() {
    assert!(SqlServerDriver.row_type_converters().is_empty());
}
