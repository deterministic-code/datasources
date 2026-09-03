use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::rusqlite_driver::RusqliteDriver;
use super::statement_builder::{SqliteColumn, SqliteStatementBuilder};
use datasources::{Connection, Statement};
use datasources_sql_specs::RepositoryDialect;
use std::sync::Arc;

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "sqlite",
        quote: SqliteStatementBuilder::quote,
        create_table: |table, columns| {
            let mapped: Vec<SqliteColumn> = columns
                .iter()
                .map(|column| SqliteColumn::new(&column.name, &column.type_name, column.id))
                .collect();
            SqliteStatementBuilder::create_table(table, &mapped)
                .expect("create table")
                .get_text()
        },
        field: super::converters::field,
        register: super::converters::register,
    }
}

#[tokio::test]
async fn user_crud() {
    for path in datasources_sql_specs::files() {
        let conn = Arc::new(SqliteConnection::new(
            Some(":memory:".into()),
            Arc::new(RusqliteDriver::new()),
        ));
        conn.open().await.unwrap();
        let command = Arc::new(SqliteCommand::new(conn.clone()));
        datasources_sql_specs::run(&path, &dialect(), conn, command)
            .await
            .unwrap();
    }
}
