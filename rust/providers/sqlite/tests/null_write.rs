use chrono::{TimeZone, Utc};
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Connection, ICRUDRepository, Row, RowTypeConverter, Value,
};
use std::sync::Arc;

use crate::connection::SqliteConnection;

fn item_converter() -> RowTypeConverter {
    RowTypeConverter::new(vec![
        "id".into(),
        "name".into(),
        "created".into(),
        "version".into(),
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
    options.entity = "item".into();
    options.row_converter = Some(item_converter());
    let repo = CRUDRepository::new(conn, options).unwrap();
    let mut omitted = Row::new();
    omitted.insert("name".into(), Value::from("ada"));
    omitted.insert("created".into(), Value::Null);
    omitted.insert("version".into(), Value::Null);
    let mut omitted = repo.create(&mut omitted).await.unwrap();
    assert!(matches!(omitted.get("id"), Some(Value::Int(id)) if *id > 0));
    let stored = repo
        .read_single_by_id(omitted.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    omitted.insert("name".into(), Value::from("ada-v2"));
    let updated = repo.update(&omitted, None).await.unwrap();
    assert_eq!(updated.rows_affected, 1);
    let kept = repo
        .read_single_by_id(omitted.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();
    let created = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
    let mut explicit = Row::new();
    explicit.insert("name".into(), Value::from("grace"));
    explicit.insert("created".into(), Value::DateTime(created));
    explicit.insert("version".into(), Value::Binary(vec![9, 9]));
    let explicit = repo.create(&mut explicit).await.unwrap();
    assert!(matches!(explicit.get("id"), Some(Value::Int(id)) if *id > 0));
    let written = repo
        .read_single_by_id(explicit.get("id").unwrap())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        stored.get("created"),
        Some(&Value::from("2020-01-01T00:00:00.000Z"))
    );
    assert_eq!(stored.get("version"), Some(&Value::Binary(vec![1, 2])));
    assert_eq!(kept.get("name"), Some(&Value::from("ada-v2")));
    assert_eq!(kept.get("created"), stored.get("created"));
    assert_eq!(kept.get("version"), stored.get("version"));
    assert_eq!(
        written.get("created"),
        Some(&Value::Text(Value::iso_datetime(created)))
    );
    assert_eq!(written.get("version"), Some(&Value::Binary(vec![9, 9])));
}
