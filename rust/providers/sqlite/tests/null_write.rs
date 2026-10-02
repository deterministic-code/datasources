use chrono::{TimeZone, Utc};
use datasources::{
    Casing, CRUDRepository, CRUDRepositoryOptions, ChangeCasingMapper, Connection, ICRUDRepository,
    Row, RowTypeConverter, Value,
};
use std::sync::Arc;

use crate::connection::SqliteConnection;

fn item_converter() -> RowTypeConverter {
    RowTypeConverter::new(vec![
        "Id".into(),
        "Name".into(),
        "Created".into(),
        "Version".into(),
    ])
}

#[tokio::test]
async fn omits_null_datetime_and_version() {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    datasources_sql_specs::apply_schema(conn.as_ref(), "null-write-item")
        .await
        .unwrap();
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Item".into();
    options.entity_mapper = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.ids = vec!["Id".into()];
    options.row_converter = Some(item_converter());
    let repo = CRUDRepository::new(conn, options).unwrap();
    let mut omitted = Row::new();
    omitted.insert("Name".into(), Value::from("ada"));
    omitted.insert("Created".into(), Value::Null);
    omitted.insert("Version".into(), Value::Null);
    let mut omitted = repo.create(&mut omitted).await.unwrap();
    assert!(matches!(omitted.get("Id"), Some(Value::Int(id)) if *id > 0));
    let stored = repo
        .read_single_by_id(omitted.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();
    omitted.insert("Name".into(), Value::from("ada-v2"));
    let updated = repo.update(&omitted, None).await.unwrap();
    assert_eq!(updated.rows_affected, 1);
    let kept = repo
        .read_single_by_id(omitted.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let created = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
    let mut explicit = Row::new();
    explicit.insert("Name".into(), Value::from("grace"));
    explicit.insert("Created".into(), Value::DateTime(created));
    explicit.insert("Version".into(), Value::Binary(vec![9, 9]));
    let explicit = repo.create(&mut explicit).await.unwrap();
    assert!(matches!(explicit.get("Id"), Some(Value::Int(id)) if *id > 0));
    let written = repo
        .read_single_by_id(explicit.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        stored.get("Created"),
        Some(&Value::from("2020-01-01T00:00:00.000Z"))
    );
    assert_eq!(stored.get("Version"), Some(&Value::Binary(vec![1, 2])));
    assert_eq!(kept.get("Name"), Some(&Value::from("ada-v2")));
    assert_eq!(kept.get("Created"), stored.get("Created"));
    assert_eq!(kept.get("Version"), stored.get("Version"));
    assert_eq!(
        written.get("Created"),
        Some(&Value::Text(Value::iso_datetime(created)))
    );
    assert_eq!(written.get("Version"), Some(&Value::Binary(vec![9, 9])));
}
