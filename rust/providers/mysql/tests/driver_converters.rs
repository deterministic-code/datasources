use crate::MysqlDriver;
use datasources::{Driver, FieldSize, SpecField};

#[test]
fn type_converter() {
    let driver = MysqlDriver;
    let types = driver.type_converter().expect("type map");
    assert_eq!(types.convert(&"string".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"character".into()).unwrap(), "CHAR(1)");
    assert_eq!(types.convert(&"integer".into()).unwrap(), "INT");
    assert_eq!(types.convert(&"unsignedinteger".into()).unwrap(), "INT UNSIGNED");
    assert_eq!(types.convert(&"datetime".into()).unwrap(), "DATETIME(3)");
    assert_eq!(types.convert(&"uuid".into()).unwrap(), "CHAR(36)");
    assert_eq!(types.convert(&"boolean".into()).unwrap(), "TINYINT(1)");
    assert_eq!(
        MysqlDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..Default::default()
        })
        .unwrap(),
        "VARCHAR(64)"
    );
    assert_eq!(
        MysqlDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(20000)),
            ..Default::default()
        })
        .unwrap(),
        "LONGTEXT"
    );
    assert_eq!(
        MysqlDriver::convert_type(&SpecField {
            type_name: "decimal".into(),
            size: Some(FieldSize::Decimal(8, 3)),
            ..Default::default()
        })
        .unwrap(),
        "DECIMAL(8, 3)"
    );
    assert_eq!(types.reverse_convert(&"DATETIME(3)".into()).unwrap(), "datetime");
    assert!(types.convert(&"nope".into()).is_err());
}

#[test]
fn default_value_converter() {
    let defaults = MysqlDriver.default_value_converter();
    assert_eq!(defaults.convert("datetime", "Now").unwrap().unwrap(), "(NOW(3))");
    assert_eq!(
        defaults.convert("datetime", "UtcNow").unwrap().unwrap(),
        "(UTC_TIMESTAMP(3))"
    );
    assert_eq!(defaults.convert("uuid", "NewId").unwrap().unwrap(), "(UUID())");
    assert_eq!(defaults.convert("binary", "Hex('ab')").unwrap().unwrap(), "(X'ab')");
    assert_eq!(defaults.convert("boolean", "true").unwrap().unwrap(), "1");
    assert_eq!(defaults.convert("string", "hi").unwrap().unwrap(), "('hi')");
    assert_eq!(defaults.reverse_convert("uuid", "(UUID())").unwrap(), "NewId");
    assert_eq!(defaults.reverse_convert("binary", "(X'ab')").unwrap(), "Hex('ab')");
    assert_eq!(defaults.reverse_convert("binary", "X'ab'").unwrap(), "Hex('ab')");
    assert!(defaults.convert("nope", "x").is_err());
}

#[test]
fn row_type_converters() {
    let catalog = MysqlDriver.row_type_converters();
    for key in [
        "boolean", "datetime", "uuid", "decimal", "binary", "integer", "unsignedinteger",
        "smallinteger", "unsignedsmallinteger", "biginteger", "unsignedbiginteger", "string",
    ] {
        assert!(catalog.contains_key(key), "{key}");
    }
}
