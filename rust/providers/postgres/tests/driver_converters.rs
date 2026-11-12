use crate::PostgresDriver;
use datasources::{Driver, FieldSize, SpecField};

#[test]
fn type_converter() {
    let driver = PostgresDriver;
    let types = driver.type_converter().expect("type map");
    assert_eq!(types.convert(&"string".into()).unwrap(), "TEXT");
    assert_eq!(types.convert(&"integer".into()).unwrap(), "INTEGER");
    assert_eq!(types.convert(&"unsignedinteger".into()).unwrap(), "BIGINT");
    assert_eq!(types.convert(&"datetime".into()).unwrap(), "TIMESTAMPTZ");
    assert_eq!(types.convert(&"uuid".into()).unwrap(), "UUID");
    assert_eq!(types.convert(&"boolean".into()).unwrap(), "BOOLEAN");
    assert_eq!(types.convert(&"binary".into()).unwrap(), "BYTEA");
    assert_eq!(
        PostgresDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..Default::default()
        })
        .unwrap(),
        "VARCHAR(64)"
    );
    assert_eq!(types.reverse_convert(&"TIMESTAMPTZ".into()).unwrap(), "datetime");
    assert!(types.convert(&"nope".into()).is_err());
}

#[test]
fn default_value_converter() {
    let defaults = PostgresDriver.default_value_converter();
    assert_eq!(defaults.convert("datetime", "Now").unwrap().unwrap(), "LOCALTIMESTAMP");
    assert_eq!(
        defaults.convert("datetime", "UtcNow").unwrap().unwrap(),
        "(NOW() AT TIME ZONE 'UTC')"
    );
    assert_eq!(defaults.convert("uuid", "NewId").unwrap().unwrap(), "gen_random_uuid()");
    assert_eq!(defaults.convert("binary", "Hex('ab')").unwrap().unwrap(), "'\\xab'");
    assert_eq!(defaults.convert("boolean", "true").unwrap().unwrap(), "TRUE");
    assert_eq!(defaults.convert("boolean", "false").unwrap().unwrap(), "FALSE");
    assert_eq!(
        defaults.reverse_convert("datetime", "LOCALTIMESTAMP").unwrap(),
        "Now"
    );
    assert_eq!(defaults.reverse_convert("uuid", "gen_random_uuid()").unwrap(), "NewId");
    assert_eq!(defaults.reverse_convert("binary", "'\\xab'").unwrap(), "Hex('ab')");
    assert!(defaults.convert("nope", "x").is_err());
}

#[test]
fn row_type_converters() {
    let catalog = PostgresDriver.row_type_converters();
    for key in [
        "boolean", "datetime", "uuid", "decimal", "binary", "integer", "unsignedinteger",
        "smallinteger", "unsignedsmallinteger", "biginteger", "unsignedbiginteger", "string",
    ] {
        assert!(catalog.contains_key(key), "{key}");
    }
}
