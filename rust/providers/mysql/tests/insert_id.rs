use super::command::MysqlCommand;
use super::connection::MysqlConnection;
use super::isolate;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Command, Connection, DictionaryMapper, InsertStatement,
    Native, Parameter, Row, Value,
};
use std::sync::Arc;

#[tokio::test]
async fn execute_reader_enumerates_result_sets() {
    let isolated = isolate::create().await;
    let conn = Arc::new(MysqlConnection::new(Some(isolated.url.clone())));
    conn.open().await.unwrap();
    let command = MysqlCommand::new(conn.clone());
    let reader = command
        .execute_reader_native(&Native::new("SELECT 1 AS id; SELECT 2 AS id"), None)
        .await
        .unwrap();
    let mut first = Vec::new();
    while reader.next().await.unwrap() {
        first.push(reader.current());
    }
    let moved = reader.next_result().await.unwrap();
    let mut second = Vec::new();
    while reader.next().await.unwrap() {
        second.push(reader.current());
    }
    let again = reader.next_result().await.unwrap();
    assert!(moved);
    assert!(!again);
    assert_eq!(first[0].as_ref().unwrap().get("id"), Some(&Value::Int(1)));
    assert_eq!(second[0].as_ref().unwrap().get("id"), Some(&Value::Int(2)));
    conn.close().await.ok();
    isolated.drop().await;
}

#[tokio::test]
async fn execute_insert_statement_returning_sets_result() {
    let isolated = isolate::create().await;
    let conn = Arc::new(MysqlConnection::new(Some(isolated.url.clone())));
    conn.open().await.unwrap();
    let command = MysqlCommand::new(conn.clone());
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE items (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT)",
            ),
            None,
        )
        .await
        .unwrap();
    let native = command
        .execute_native(&Native::new("INSERT INTO items (name) VALUES ('ada')"), None)
        .await
        .unwrap();
    assert_eq!(native.result, Value::Null);
    let fields = [Parameter::new("name", "bea")];
    let insert = InsertStatement::new("items", &fields).returning("id");
    let inserted = command
        .execute_statement(&insert, Some(insert.fields()))
        .await
        .unwrap();
    assert_ne!(inserted.result, Value::Null);
    assert_eq!(inserted.result, Value::Int(2));
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["id".into()];
    options.entity = "items".into();
    let repo = CRUDRepository::<Row>::new(conn.clone(), options).unwrap();
    let mut cee = Row::new();
    cee.insert("name".into(), Value::from("cee"));
    let created = repo.create(cee).await.unwrap();
    let created_id = match created.get("id") {
        Some(Value::Int(id)) => *id,
        other => panic!("expected id, got {other:?}"),
    };
    assert!(created_id > 0);
    let mut dee = Row::new();
    dee.insert("name".into(), Value::from("dee"));
    let next = repo.create(dee).await.unwrap();
    let next_id = match next.get("id") {
        Some(Value::Int(id)) => *id,
        other => panic!("expected id, got {other:?}"),
    };
    assert!(next_id > created_id);
    conn.close().await.ok();
    isolated.drop().await;
}

#[tokio::test]
async fn create_then_child_insert_uses_the_parent_id() {
    let isolated = isolate::create().await;
    let conn = Arc::new(MysqlConnection::new(Some(isolated.url.clone())));
    conn.open().await.unwrap();
    let command = MysqlCommand::new(conn.clone());
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT NOT NULL); CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTO_INCREMENT, contact_id INTEGER NOT NULL, line1 TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    let mut contacts_options = CRUDRepositoryOptions::default();
    contacts_options.ids = vec!["id".into()];
    contacts_options.entity = "contacts".into();
    contacts_options.create_properties = Some(vec!["name".into()]);
    let contacts = CRUDRepository::<Row>::new(conn.clone(), contacts_options).unwrap();
    let mut addresses_options = CRUDRepositoryOptions::default();
    addresses_options.ids = vec!["id".into()];
    addresses_options.entity = "addresses".into();
    addresses_options.create_properties = Some(vec!["contactId".into(), "line1".into()]);
    addresses_options.property_mappings = Some(Arc::new(DictionaryMapper::new([(
        "contactId".to_string(),
        "contact_id".to_string(),
    )])));
    let addresses = CRUDRepository::<Row>::new(conn.clone(), addresses_options).unwrap();
    let mut parent = Row::new();
    parent.insert("name".into(), Value::from("ada"));
    let created = contacts.create(parent).await.unwrap();
    let parent_id = created.get("id").cloned().expect("id");
    let mut child = Row::new();
    child.insert("contactId".into(), parent_id.clone());
    child.insert("line1".into(), Value::from("12 Engine"));
    addresses.create(child).await.unwrap();
    let reader = command
        .execute_reader_native(&Native::new("SELECT contact_id FROM addresses"), None)
        .await
        .unwrap();
    assert!(reader.next().await.unwrap());
    assert_eq!(reader.current().unwrap().get("contact_id"), Some(&parent_id));
    conn.close().await.ok();
    isolated.drop().await;
}
