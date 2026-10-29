use crate::api::Api;
use crate::clock::Clock;
use crate::repos;
use datasources::Driver;
use std::collections::HashMap;

#[tokio::test]
async fn maps_datasource_errors() {
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), "sqlite:///:memory:".into());
    let connection = datasources_sqlite::SqliteDriver.connection(Some(&options));
    connection.open().await.unwrap();
    let (contacts, legacy, groups_repo) = repos::open(connection, None).unwrap();
    let api = Api::new(contacts, legacy, groups_repo, Clock::new());
    assert_eq!(api.handle("GET", "/contacts", None).await.unwrap().status, 400);
}

#[tokio::test]
async fn clock_defaults_cover_now() {
    let _ = Clock::new().now();
    let _ = Clock::default().uuid();
}
