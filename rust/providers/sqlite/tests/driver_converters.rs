use crate::SqliteDriver;
use datasources::{Driver, FieldSize, SpecField};

#[test]
fn type_converter() {
    let driver = SqliteDriver;
    let types = driver.type_converter().expect("type map");
    assert_eq!(types.convert(&"string".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"character".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"integer".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"unsignedinteger".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"smallinteger".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"unsignedsmallinteger".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"biginteger".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"unsignedbiginteger".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"float".into()).unwrap(), "REAL");
    assert_eq!(types.convert(&"decimal".into()).unwrap(), "NUMERIC");
    assert_eq!(types.convert(&"boolean".into()).unwrap(), "BOOLEAN");
    assert_eq!(types.convert(&"datetime".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"binary".into()).unwrap(), "BLOB");
    assert_eq!(types.convert(&"uuid".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"reference".into()).unwrap(), "INTEGER");
    assert_eq!(
        SqliteDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..Default::default()
        })
        .unwrap(),
        "VARCHAR(64)"
    );
    assert_eq!(types.reverse_convert(&"REAL".into()).unwrap(), "float");
    assert!(types.convert(&"nope".into()).is_err());
}

#[test]
fn default_value_converter() {
    let defaults = SqliteDriver.default_value_converter();
    assert_eq!(
        defaults.convert("datetime", "Now").unwrap().unwrap(),
        "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))"
    );
    assert_eq!(
        defaults.convert("datetime", "UtcNow").unwrap().unwrap(),
        "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))"
    );
    assert!(defaults.convert("uuid", "NewId").unwrap().unwrap().contains("randomblob"));
    assert_eq!(defaults.convert("binary", "Hex('ab')").unwrap().unwrap(), "X'ab'");
    assert_eq!(defaults.convert("boolean", "true").unwrap().unwrap(), "1");
    assert_eq!(defaults.convert("boolean", "false").unwrap().unwrap(), "0");
    assert_eq!(defaults.convert("string", "hi").unwrap().unwrap(), "'hi'");
    assert_eq!(
        defaults
            .reverse_convert("datetime", "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))")
            .unwrap(),
        "UtcNow"
    );
    assert_eq!(defaults.reverse_convert("binary", "X'ab'").unwrap(), "Hex('ab')");
    assert!(defaults.convert("nope", "x").is_err());
}

#[test]
fn row_type_converters() {
    let catalog = SqliteDriver.row_type_converters();
    for key in [
        "boolean",
        "datetime",
        "uuid",
        "decimal",
        "binary",
        "integer",
        "unsignedinteger",
        "smallinteger",
        "unsignedsmallinteger",
        "biginteger",
        "unsignedbiginteger",
        "string",
    ] {
        assert!(catalog.contains_key(key), "{key}");
    }
}
