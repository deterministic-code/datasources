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
