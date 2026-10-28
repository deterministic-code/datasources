#![allow(unused)]

#[path = "../src/datasources_register.rs"]
mod datasources_register;
use datasources_register::Repositories;

use datasources::{
    BuiltDatasource, Datasource, ICRUDRepository, IReadonlyRepository, Operator, Row, Value, Where,
};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut options = HashMap::new();
    options.insert(
        "DATASOURCE_URL".into(),
        std::env::var("DATASOURCE_URL").expect("DATASOURCE_URL"),
    );
    let ds = BuiltDatasource::new(
        "sqlite",
        Arc::new(datasources_sqlite::SqliteDriver),
        Some(options),
    );
    let ds: &dyn Datasource = &ds;
    let connection = ds.create_connection().await.expect("connection");
    connection.open().await.expect("open");
    let contacts = ds.contacts();
    let mut row = Row::new();
    row.insert("first_name".into(), Value::from("Ada"));
    row.insert("last_name".into(), Value::from("Lovelace"));
    row.insert("contact_source_id".into(), Value::from(1_i64));
    row.insert(
        "uuid".into(),
        Value::from("00000000-0000-0000-0000-000000000001"),
    );
    row.insert("version".into(), Value::from(vec![0u8]));
    let created: Row = contacts.create(row).await.expect("create");
    assert_eq!(
        created.get("first_name").map(Value::wire_text).as_deref(),
        Some("Ada")
    );
    let id = match created.get("id") {
        Some(Value::Int(id)) => *id,
        _ => panic!("id"),
    };
    let loaded: Option<Row> = contacts
        .read_single(Some(&Where::new(Operator::equals("id", id))), None, None)
        .await
        .expect("read");
    let loaded = loaded.expect("row");
    assert_eq!(
        loaded.get("first_name").map(Value::wire_text).as_deref(),
        Some("Ada")
    );
}
