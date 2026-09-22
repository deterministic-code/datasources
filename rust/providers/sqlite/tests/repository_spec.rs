use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::rusqlite_driver::RusqliteDriver;
use super::{SqliteColumnType, SqliteQuote};
use datasources::{Column, Connection, CreateTableStatement, Text};
use datasources_sql_specs::RepositoryDialect;
use std::sync::Arc;

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "sqlite",
        quote: SqliteQuote::quote,
        create_table: |table, columns| {
            let mapped: Vec<Column> = columns
                .iter()
                .map(|column| {
                    let affinity = SqliteColumnType::map(&column.type_name).expect("column type");
                    Column::new(
                        SqliteQuote::quote(&column.name),
                        if column.id {
                            format!("{affinity} PRIMARY KEY")
                        } else {
                            affinity
                        },
                    )
                })
                .collect();
            CreateTableStatement::new(SqliteQuote::quote(table), &mapped, &[]).get_text()
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
