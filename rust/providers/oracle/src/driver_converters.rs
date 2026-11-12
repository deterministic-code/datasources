use crate::OracleDriver;
use datasources::{Driver, FieldSize, SpecField};

#[test]
fn type_converter() {
    let driver = OracleDriver;
    let types = driver.type_converter().expect("type map");
    assert_eq!(types.convert(&"string".into()).unwrap(), "CLOB");
    assert_eq!(types.convert(&"integer".into()).unwrap(), "NUMBER(10)");
    assert_eq!(types.convert(&"datetime".into()).unwrap(), "TIMESTAMP");
    assert_eq!(types.convert(&"uuid".into()).unwrap(), "VARCHAR2(36)");
    assert_eq!(types.convert(&"binary".into()).unwrap(), "BLOB");
    assert_eq!(
        OracleDriver::convert_type(&SpecField {
            type_name: "string".into(),
            size: Some(FieldSize::Length(64)),
            ..Default::default()
        })
        .unwrap(),
        "VARCHAR2(64)"
    );
    assert_eq!(
        OracleDriver::convert_type(&SpecField {
            type_name: "binary".into(),
            size: Some(FieldSize::Length(16)),
            ..Default::default()
        })
        .unwrap(),
        "RAW(16)"
    );
    assert_eq!(types.reverse_convert(&"TIMESTAMP".into()).unwrap(), "datetime");
    assert!(types.convert(&"nope".into()).is_err());
}

#[test]
fn default_value_converter() {
    let defaults = OracleDriver.default_value_converter();
    assert_eq!(defaults.convert("datetime", "Now").unwrap().unwrap(), "LOCALTIMESTAMP");
    assert_eq!(
        defaults.convert("datetime", "UtcNow").unwrap().unwrap(),
        "SYS_EXTRACT_UTC(SYSTIMESTAMP)"
    );
    assert_eq!(defaults.convert("uuid", "NewId").unwrap().unwrap(), "SYS_GUID()");
    assert_eq!(
        defaults.convert("binary", "Hex('ab')").unwrap().unwrap(),
        "HEXTORAW('ab')"
    );
    assert_eq!(defaults.convert("boolean", "true").unwrap().unwrap(), "1");
    assert_eq!(defaults.reverse_convert("uuid", "SYS_GUID()").unwrap(), "NewId");
    assert_eq!(
        defaults.reverse_convert("binary", "HEXTORAW('ab')").unwrap(),
        "Hex('ab')"
    );
    assert!(defaults.convert("nope", "x").is_err());
}

#[test]
fn empty_row_type_converters() {
    assert!(OracleDriver.row_type_converters().is_empty());
}
