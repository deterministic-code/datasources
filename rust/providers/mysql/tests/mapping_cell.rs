use chrono::{TimeZone, Utc};
use datasources::{Driver, Row, RowTypeConverter, Value};
use std::collections::HashMap;

use crate::MysqlDriver;

fn columns() -> HashMap<String, String> {
    HashMap::from([
        ("TextToInteger".into(), "integer".into()),
        ("TextToDecimal".into(), "decimal".into()),
        ("TextToDatetime".into(), "datetime".into()),
        ("FloatToInteger".into(), "integer".into()),
        ("DecimalToInteger".into(), "integer".into()),
        ("DecimalToDecimal".into(), "decimal".into()),
        ("DatetimeToDatetime".into(), "datetime".into()),
        ("IntToBoolean".into(), "boolean".into()),
        ("IntToInteger".into(), "integer".into()),
    ])
}

#[test]
fn converts_mapping_cell_driver_cells() {
    let opened = Utc.with_ymd_and_hms(2020, 1, 2, 3, 4, 5).unwrap();
    let converter = RowTypeConverter::from(|| MysqlDriver.row_type_converters(), &columns());
    let converted = converter
        .convert(&Row::from([
            ("TextToInteger".into(), Value::from("10")),
            ("TextToDecimal".into(), Value::from("10.50")),
            (
                "TextToDatetime".into(),
                Value::from("2020-01-02T03:04:05.000Z"),
            ),
            ("FloatToInteger".into(), Value::Float(10.0)),
            (
                "DecimalToDecimal".into(),
                Value::from("10.50000000000000"),
            ),
            ("DatetimeToDatetime".into(), Value::DateTime(opened)),
            ("IntToBoolean".into(), Value::Int(1)),
        ]))
        .unwrap();
    assert_eq!(converted.get("TextToInteger"), Some(&Value::Int(10)));
    assert_eq!(converted.get("TextToDecimal"), Some(&Value::from("10.5")));
    assert!(matches!(
        converted.get("TextToDatetime"),
        Some(Value::DateTime(_))
    ));
    assert_eq!(converted.get("FloatToInteger"), Some(&Value::Int(10)));
    assert_eq!(
        converted.get("DecimalToDecimal"),
        Some(&Value::from("10.5"))
    );
    assert_eq!(converted.get("IntToBoolean"), Some(&Value::Bool(true)));
    let stored = converter
        .reverse_convert(&Row::from([
            ("DatetimeToDatetime".into(), Value::DateTime(opened)),
            ("DecimalToDecimal".into(), Value::from("10.50")),
        ]))
        .unwrap();
    assert_eq!(
        stored.get("DatetimeToDatetime"),
        Some(&Value::from("2020-01-02 03:04:05.000"))
    );
    assert_eq!(stored.get("DecimalToDecimal"), Some(&Value::from("10.5")));
}

#[test]
fn rejects_dotted_integer_text_on_mapping_cell() {
    let converter = RowTypeConverter::from(|| MysqlDriver.row_type_converters(), &columns());
    assert_eq!(
        converter
            .convert(&Row::from([("TextToInteger".into(), Value::from("10.0"))]))
            .unwrap()
            .get("TextToInteger"),
        Some(&Value::Int(10))
    );
    assert_eq!(
        converter
            .convert(&Row::from([("FloatToInteger".into(), Value::from("10.0"))]))
            .unwrap()
            .get("FloatToInteger"),
        Some(&Value::Int(10))
    );
    assert_eq!(
        converter
            .convert(&Row::from([(
                "DecimalToInteger".into(),
                Value::from("10.00000000000000")
            )]))
            .unwrap()
            .get("DecimalToInteger"),
        Some(&Value::Int(10))
    );
    assert!(converter
        .convert(&Row::from([(
            "DecimalToInteger".into(),
            Value::from("10.50000000000000")
        )]))
        .is_err());
}

#[test]
fn converts_and_reverses_the_mapping_cell_matrix() {
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    let token = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = Uuid::parse_str(token).unwrap();
    let opened = Utc.with_ymd_and_hms(2020, 1, 2, 3, 4, 5).unwrap();
    let integral = "10.00000000000000".parse::<Decimal>().unwrap();
    let fractional = "10.50000000000000".parse::<Decimal>().unwrap();
    let wide = "1.2345678901234567".parse::<Decimal>().unwrap();
    let columns = HashMap::from([
        ("TextToString".into(), "string".into()),
        ("TextToInteger".into(), "integer".into()),
        ("TextToDecimal".into(), "decimal".into()),
        ("TextToDatetime".into(), "datetime".into()),
        ("TextToBinary".into(), "binary".into()),
        ("TextToGuid".into(), "uuid".into()),
        ("IntToInteger".into(), "integer".into()),
        ("IntToFloat".into(), "float".into()),
        ("IntToDecimal".into(), "decimal".into()),
        ("IntToBoolean".into(), "boolean".into()),
        ("FloatToInteger".into(), "integer".into()),
        ("FloatToFloat".into(), "float".into()),
        ("FloatToDecimal".into(), "decimal".into()),
        ("DecimalToInteger".into(), "integer".into()),
        ("DecimalToFloat".into(), "float".into()),
        ("DecimalToDecimal".into(), "decimal".into()),
        ("BoolToBoolean".into(), "boolean".into()),
        ("DatetimeToDatetime".into(), "datetime".into()),
        ("BinaryToBinary".into(), "binary".into()),
        ("BinaryToGuid".into(), "uuid".into()),
        ("GuidToString".into(), "string".into()),
        ("GuidToGuid".into(), "uuid".into()),
    ]);
    let converter = RowTypeConverter::from(|| MysqlDriver.row_type_converters(), &columns);
    let converted = converter
        .convert(&Row::from([
            ("TextToString".into(), Value::from("ada")),
            ("TextToInteger".into(), Value::from("10")),
            ("TextToDecimal".into(), Value::from("10.50")),
            ("TextToDatetime".into(), Value::from("2020-01-02T03:04:05.000Z")),
            ("TextToBinary".into(), Value::from("not-base64")),
            ("TextToGuid".into(), Value::from(token)),
            ("IntToInteger".into(), Value::Int(10)),
            ("IntToFloat".into(), Value::Int(10)),
            ("IntToDecimal".into(), Value::Int(7)),
            ("IntToBoolean".into(), Value::Int(1)),
            ("FloatToInteger".into(), Value::Float(10.0)),
            ("FloatToFloat".into(), Value::Float(10.9)),
            ("FloatToDecimal".into(), Value::Float(10.5)),
            ("DecimalToInteger".into(), Value::Decimal(integral)),
            ("DecimalToFloat".into(), Value::from("1.2345678901234567")),
            ("DecimalToDecimal".into(), Value::Decimal(fractional)),
            ("BoolToBoolean".into(), Value::Int(0)),
            ("DatetimeToDatetime".into(), Value::DateTime(opened)),
            ("BinaryToBinary".into(), Value::Binary(vec![1, 2, 3, 4])),
            ("BinaryToGuid".into(), Value::Binary(token.as_bytes().to_vec())),
            ("GuidToString".into(), Value::from(token)),
            ("GuidToGuid".into(), Value::Uuid(uuid)),
        ]))
        .unwrap();
    assert_eq!(converted.get("TextToString"), Some(&Value::from("ada")));
    assert_eq!(converted.get("TextToInteger"), Some(&Value::Int(10)));
    assert_eq!(converted.get("TextToDecimal"), Some(&Value::from("10.5")));
    assert_eq!(converted.get("TextToDatetime"), Some(&Value::DateTime(opened)));
    assert_eq!(
        converted.get("TextToBinary"),
        Some(&Value::Binary(b"not-base64".to_vec()))
    );
    assert_eq!(converted.get("TextToGuid"), Some(&Value::from(token)));
    assert_eq!(converted.get("IntToInteger"), Some(&Value::Int(10)));
    assert_eq!(converted.get("IntToFloat"), Some(&Value::Float(10.0)));
    assert_eq!(converted.get("IntToDecimal"), Some(&Value::from("7")));
    assert_eq!(converted.get("IntToBoolean"), Some(&Value::Bool(true)));
    assert_eq!(converted.get("FloatToInteger"), Some(&Value::Int(10)));
    assert_eq!(converted.get("FloatToFloat"), Some(&Value::Float(10.9)));
    assert_eq!(converted.get("FloatToDecimal"), Some(&Value::from("10.5")));
    assert_eq!(converted.get("DecimalToInteger"), Some(&Value::Int(10)));
    assert_eq!(
        converted.get("DecimalToFloat"),
        Some(&Value::Float("1.2345678901234567".parse().unwrap()))
    );
    assert_eq!(converted.get("DecimalToDecimal"), Some(&Value::from("10.5")));
    assert_eq!(converted.get("BoolToBoolean"), Some(&Value::Bool(false)));
    assert_eq!(converted.get("DatetimeToDatetime"), Some(&Value::DateTime(opened)));
    assert_eq!(
        converted.get("BinaryToBinary"),
        Some(&Value::Binary(vec![1, 2, 3, 4]))
    );
    assert_eq!(converted.get("BinaryToGuid"), Some(&Value::from(token)));
    assert_eq!(converted.get("GuidToString"), Some(&Value::from(token)));
    assert_eq!(converted.get("GuidToGuid"), Some(&Value::from(token)));
    assert_eq!(
        converter
            .convert(&Row::from([(
                "DecimalToFloat".into(),
                Value::Decimal(wide)
            )]))
            .unwrap()
            .get("DecimalToFloat"),
        Some(&Value::Float(wide.to_f64().unwrap()))
    );
    assert_eq!(
        converter
            .convert(&Row::from([(
                "BinaryToGuid".into(),
                Value::Binary(uuid.as_bytes().to_vec())
            )]))
            .unwrap()
            .get("BinaryToGuid"),
        Some(&Value::from(token))
    );
    assert!(converter
        .convert(&Row::from([("DecimalToInteger".into(), Value::Decimal(fractional))]))
        .is_err());
    let stored = converter
        .reverse_convert(&Row::from([
            ("IntToInteger".into(), Value::Int(10)),
            ("IntToFloat".into(), Value::Float(10.0)),
            ("IntToBoolean".into(), Value::Bool(true)),
            ("FloatToInteger".into(), Value::Int(10)),
            ("FloatToDecimal".into(), Value::Decimal(fractional)),
            ("DecimalToInteger".into(), Value::Int(10)),
            ("DecimalToFloat".into(), Value::Float(wide.to_f64().unwrap())),
            ("DecimalToDecimal".into(), Value::Decimal(fractional)),
            ("BoolToBoolean".into(), Value::Bool(false)),
            ("DatetimeToDatetime".into(), Value::DateTime(opened)),
            ("BinaryToGuid".into(), Value::Uuid(uuid)),
            ("GuidToGuid".into(), Value::Uuid(uuid)),
        ]))
        .unwrap();
    assert_eq!(stored.get("IntToInteger"), Some(&Value::Int(10)));
    assert_eq!(stored.get("IntToFloat"), Some(&Value::Float(10.0)));
    assert_eq!(stored.get("IntToBoolean"), Some(&Value::Int(1)));
    assert_eq!(stored.get("FloatToInteger"), Some(&Value::Int(10)));
    assert_eq!(stored.get("FloatToDecimal"), Some(&Value::from("10.5")));
    assert_eq!(stored.get("DecimalToInteger"), Some(&Value::Int(10)));
    assert_eq!(
        stored.get("DecimalToFloat"),
        Some(&Value::Float(wide.to_f64().unwrap()))
    );
    assert_eq!(stored.get("DecimalToDecimal"), Some(&Value::from("10.5")));
    assert_eq!(stored.get("BoolToBoolean"), Some(&Value::Int(0)));
    assert_eq!(
        stored.get("DatetimeToDatetime"),
        Some(&Value::from("2020-01-02 03:04:05.000"))
    );
    assert_eq!(stored.get("BinaryToGuid"), Some(&Value::from(token)));
    assert_eq!(stored.get("GuidToGuid"), Some(&Value::from(token)));
}
