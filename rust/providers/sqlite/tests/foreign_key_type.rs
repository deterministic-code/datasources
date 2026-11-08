use datasources::{
    Casing, CRUDRepository, CRUDRepositoryOptions, ChangeCasingMapper, Connection, Native, Row, Schema,
    Value,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::connection::SqliteConnection;

#[tokio::test]
async fn foreign_key_matches_long_parent_id() {
    let (parent_id, loaded) = roundtrip(
        "CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        "CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id INTEGER NOT NULL, city TEXT NOT NULL)",
        "integer",
        None,
    )
    .await;
    assert!(matches!(parent_id, Value::Int(_)));
    assert_eq!(loaded.get("ContactId"), Some(&parent_id));
}

#[tokio::test]
async fn foreign_key_matches_guid_parent_id() {
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let (parent_id, loaded) = roundtrip(
        "CREATE TABLE contacts (id TEXT PRIMARY KEY, name TEXT NOT NULL)",
        "CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id TEXT NOT NULL, city TEXT NOT NULL)",
        "uuid",
        Some(Value::Uuid(id)),
    )
    .await;
    match loaded.get("ContactId") {
        Some(Value::Uuid(got)) => assert_eq!(*got, id),
        Some(Value::Text(got)) => assert_eq!(got, &id.to_string()),
        other => panic!("{other:?}"),
    }
    match parent_id {
        Value::Uuid(got) => assert_eq!(got, id),
        Value::Text(got) => assert_eq!(got, id.to_string()),
        other => panic!("{other:?}"),
    }
}

async fn roundtrip(
    contacts_sql: &str,
    addresses_sql: &str,
    id_type: &str,
    parent_id: Option<Value>,
) -> (Value, Row) {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    let command = conn.create_command();
    command.execute_native(&Native::new(contacts_sql), None).await.unwrap();
    command.execute_native(&Native::new(addresses_sql), None).await.unwrap();
    let casing = std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let mut contacts_options = CRUDRepositoryOptions::default();
    contacts_options.entity = "contacts".into();
    contacts_options.property_mappings = Some(casing.clone());
    contacts_options.ids = vec!["Id".into()];
    contacts_options.schema = Schema::new(HashMap::from([
        ("Id".into(), id_type.into()),
        ("Name".into(), "string".into()),
    ]));
    if parent_id.is_some() {
        contacts_options.create_properties = Some(vec!["Id".into(), "Name".into()]);
    }
    let mut addresses_options = CRUDRepositoryOptions::default();
    addresses_options.entity = "addresses".into();
    addresses_options.property_mappings = Some(casing);
    addresses_options.ids = vec!["Id".into()];
    addresses_options.schema = Schema::new(HashMap::from([
        ("Id".into(), "integer".into()),
        ("ContactId".into(), id_type.into()),
        ("City".into(), "string".into()),
    ]));
    let contacts_repo = CRUDRepository::<Row>::new(conn.clone(), contacts_options).unwrap();
    let addresses_repo = CRUDRepository::<Row>::new(conn, addresses_options).unwrap();
    let mut parent_row = Row::new();
    parent_row.insert("Name".into(), Value::from("Ada"));
    if let Some(id) = parent_id {
        parent_row.insert("Id".into(), id);
    }
    let parent = contacts_repo.create(parent_row).await.unwrap();
    let parent_id = parent.get("Id").cloned().unwrap();
    let mut child_row = Row::new();
    child_row.insert("ContactId".into(), parent_id.clone());
    child_row.insert("City".into(), Value::from("London"));
    let child = addresses_repo.create(child_row).await.unwrap();
    let loaded = addresses_repo
        .read_single_by_id(child.get("Id").unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(child.get("ContactId"), Some(&parent_id));
    (parent_id, loaded)
}
