use super::command::PostgresCommand;
use super::connection::PostgresConnection;
use super::statement_builder::{PostgresColumn, PostgresStatementBuilder};
use super::tokio_postgres_driver::TokioPostgresDriver;
use datasources::{Connection, Text};
use datasources_sql_specs::RepositoryDialect;
use std::sync::Arc;
use std::time::SystemTime;
use tokio_postgres::NoTls;

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "postgres",
        quote: PostgresStatementBuilder::quote,
        create_table: |table, columns| {
            let mapped: Vec<PostgresColumn> = columns
                .iter()
                .map(|column| PostgresColumn::new(&column.name, &column.type_name, column.id))
                .collect();
            PostgresStatementBuilder::create_table(table, &mapped)
                .expect("create table")
                .get_text()
        },
        field: super::converters::field,
        register: super::converters::register,
    }
}

#[tokio::test]
async fn user_crud() {
    let Ok(url) = std::env::var("POSTGRES_URL") else {
        return;
    };
    for path in datasources_sql_specs::files() {
        let schema = format!(
            "ds_{:x}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let (client, connection) = tokio_postgres::connect(&url, NoTls).await.unwrap();
        tokio::spawn(async move {
            let _ = connection.await;
        });
        client
            .batch_execute(&format!("CREATE SCHEMA {schema}"))
            .await
            .unwrap();
        let conn = Arc::new(PostgresConnection::new(
            Some(url.clone()),
            Some(schema.clone()),
            Arc::new(TokioPostgresDriver::new(Some(schema.clone()))),
        ));
        conn.open().await.unwrap();
        let command = Arc::new(PostgresCommand::new(conn.clone()));
        datasources_sql_specs::run(&path, &dialect(), conn, command)
            .await
            .unwrap();
        client
            .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .ok();
    }
}
