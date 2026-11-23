use chrono::{TimeZone, Utc};
use datasources::{
    Casing, CRUDRepository, CRUDRepositoryOptions, ChangeCasingMapper, Connection, Driver,
    Native, Row, RowTypeConverter, Schema, SchemaColumn, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::connection::SqliteConnection;
use crate::SqliteDriver;

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
    let converter = RowTypeConverter::from(|| SqliteDriver.row_type_converters(), &columns());
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
            ("IntToInteger".into(), Value::Int(10)),
            ("DatetimeToDatetime".into(), Value::DateTime(opened)),
            ("DecimalToDecimal".into(), Value::from("10.50")),
        ]))
        .unwrap();
    assert_eq!(stored.get("IntToInteger"), Some(&Value::Int(10)));
    assert_eq!(
        stored.get("DatetimeToDatetime"),
        Some(&Value::from("2020-01-02T03:04:05.000Z"))
    );
    assert_eq!(stored.get("DecimalToDecimal"), Some(&Value::from("10.5")));
}

#[test]
fn rejects_dotted_integer_text_on_mapping_cell() {
    let converter = RowTypeConverter::from(|| SqliteDriver.row_type_converters(), &columns());
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

fn properties() -> Vec<String> {
    vec![
        "Id".into(),
        "TextToString".into(),
        "TextToInteger".into(),
        "TextToDecimal".into(),
        "TextToDatetime".into(),
        "IntToInteger".into(),
        "IntToBoolean".into(),
        "BoolToBoolean".into(),
        "DatetimeToDatetime".into(),
        "GuidToGuid".into(),
    ]
}

fn options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "MappingCell".into();
    options.entity_mapper = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.ids = vec!["Id".into()];
    options.read_properties = properties();
    options.create_properties = Some(properties());
    options.schema = Schema::new(HashMap::from([
        ("Id".into(), SchemaColumn::new("integer")),
        ("TextToString".into(), SchemaColumn::new("string")),
        ("TextToInteger".into(), SchemaColumn::new("integer")),
        ("TextToDecimal".into(), SchemaColumn::new("decimal")),
        ("TextToDatetime".into(), SchemaColumn::new("datetime")),
        ("IntToInteger".into(), SchemaColumn::new("integer")),
        ("IntToBoolean".into(), SchemaColumn::new("boolean")),
        ("BoolToBoolean".into(), SchemaColumn::new("boolean")),
        ("DatetimeToDatetime".into(), SchemaColumn::new("datetime")),
        ("GuidToGuid".into(), SchemaColumn::new("uuid")),
    ]));
    options
}

#[tokio::test]
async fn create_and_read_round_trip_mapping_cell_conversions() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    conn.create_command()
        .execute_native(
            &Native::new(
                "CREATE TABLE mapping_cell (
              id INTEGER PRIMARY KEY,
              text_to_string TEXT,
              text_to_integer TEXT,
              text_to_decimal TEXT,
              text_to_datetime TEXT,
              int_to_integer INTEGER,
              int_to_boolean INTEGER,
              bool_to_boolean INTEGER,
              datetime_to_datetime TEXT,
              guid_to_guid TEXT
            )",
            ),
            None,
        )
        .await
        .unwrap();
    let repo = CRUDRepository::<Row>::new(Arc::clone(&conn) as Arc<dyn Connection>, options()).unwrap();
    let opened = Utc.with_ymd_and_hms(2020, 1, 2, 3, 4, 5).unwrap();
    let mut seed = Row::new();
    seed.insert("Id".into(), Value::Int(1));
    seed.insert("TextToString".into(), Value::from("ada"));
    seed.insert("TextToInteger".into(), Value::Int(10));
    seed.insert("TextToDecimal".into(), Value::from("10.50"));
    seed.insert("TextToDatetime".into(), Value::DateTime(opened));
    seed.insert("IntToInteger".into(), Value::Int(10));
    seed.insert("IntToBoolean".into(), Value::Bool(true));
    seed.insert("BoolToBoolean".into(), Value::Bool(true));
    seed.insert("DatetimeToDatetime".into(), Value::DateTime(opened));
    seed.insert(
        "GuidToGuid".into(),
        Value::from("550e8400-e29b-41d4-a716-446655440000"),
    );
    let created = repo.create(seed).await.unwrap();
    let id = created.get("Id").unwrap().clone();
    let read = repo.read_single_by_id(&id).await.unwrap().unwrap();
    assert_eq!(read.get("TextToString"), Some(&Value::from("ada")));
    assert_eq!(read.get("TextToInteger"), Some(&Value::Int(10)));
    assert_eq!(read.get("TextToDecimal"), Some(&Value::from("10.5")));
    assert!(matches!(read.get("TextToDatetime"), Some(Value::DateTime(_))));
    assert_eq!(read.get("IntToInteger"), Some(&Value::Int(10)));
    assert_eq!(read.get("IntToBoolean"), Some(&Value::Bool(true)));
    assert_eq!(read.get("BoolToBoolean"), Some(&Value::Bool(true)));
    assert_eq!(
        read.get("GuidToGuid"),
        Some(&Value::from("550e8400-e29b-41d4-a716-446655440000"))
    );
}

#[tokio::test]
async fn reading_zero_fraction_dotted_text_as_integer() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    conn.create_command()
        .execute_native(
            &Native::new("CREATE TABLE mapping_cell (id INTEGER PRIMARY KEY, text_to_integer TEXT)"),
            None,
        )
        .await
        .unwrap();
    conn.create_command()
        .execute_native(
            &Native::new("INSERT INTO mapping_cell (id, text_to_integer) VALUES (1, '10.0')"),
            None,
        )
        .await
        .unwrap();
    let mut options = options();
    options.read_properties = vec!["Id".into(), "TextToInteger".into()];
    options.create_properties = Some(options.read_properties.clone());
    let repo = CRUDRepository::<Row>::new(Arc::clone(&conn) as Arc<dyn Connection>, options).unwrap();
    let read = repo.read_single_by_id(&Value::Int(1)).await.unwrap().unwrap();
    assert_eq!(read.get("TextToInteger"), Some(&Value::Int(10)));
}
