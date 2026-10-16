use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use datasources::Connection;
use datasources_tests::RepositoryDialect;
use std::sync::Arc;

fn quote(id: &str) -> String {
    format!("\"{}\"", id.replace('"', "\"\""))
}

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "sqlite",
        quote,
        field: super::converters::field,
        types: super::converters::row_type_converters,
    }
}

#[tokio::test]
async fn user_crud() {
    for path in datasources_tests::files() {
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
        conn.open().await.unwrap();
        let command = Arc::new(SqliteCommand::new(conn.clone()));
        datasources_tests::run(&path, &dialect(), conn, command)
            .await
            .unwrap();
    }
}
