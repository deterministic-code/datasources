use crate::migrate;
use crate::schema::sql_path;
use datasources::Driver;
use std::collections::HashMap;

#[tokio::test]
async fn applies_generated_sqlite_sql() {
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), "sqlite:///:memory:".into());
    let connection = datasources_sqlite::SqliteDriver.connection(Some(&options));
    connection.open().await.unwrap();
    migrate::apply(connection.create_command().as_ref(), connection.provider())
        .await
        .unwrap();
    assert!(sql_path(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
    )
    .unwrap()
    .to_string_lossy()
    .ends_with("migrations"));
}
