use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use datasources::{Column, Connection, CreateTableStatement, Text};
use datasources_sql_specs::RepositoryDialect;
use std::sync::Arc;

fn quote(id: &str) -> String {
    format!("\"{}\"", id.replace('"', "\"\""))
}

fn map(canonical: &str) -> String {
    match canonical {
        "int" => "INTEGER",
        "uuid" | "string" | "datetime" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "binary" => "BLOB",
        _ => panic!("Unknown sqlite type \"{canonical}\""),
    }
    .into()
}

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "sqlite",
        quote: quote,
        create_table: |table, columns| {
            let mapped: Vec<Column> = columns
                .iter()
                .map(|column| {
                    let affinity = map(&column.type_name);
                    Column::new(
                        quote(&column.name),
                        if column.id {
                            format!("{affinity} PRIMARY KEY")
                        } else {
                            affinity
                        },
                    )
                })
                .collect();
            CreateTableStatement::new(quote(table), &mapped, &[]).get_text()
        },
        field: super::converters::field,
        types: super::converters::row_type_converters,
    }
}

#[tokio::test]
async fn user_crud() {
    for path in datasources_sql_specs::files() {
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
        conn.open().await.unwrap();
        let command = Arc::new(SqliteCommand::new(conn.clone()));
        datasources_sql_specs::run(&path, &dialect(), conn, command)
            .await
            .unwrap();
    }
}
