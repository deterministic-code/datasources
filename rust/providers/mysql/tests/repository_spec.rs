use super::command::MysqlCommand;
use super::connection::MysqlConnection;
use datasources::{Column, Connection, CreateTableStatement, Text};
use datasources_sql_specs::RepositoryDialect;
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::sync::Arc;
use std::time::SystemTime;

fn quote(id: &str) -> String {
    format!("`{}`", id.replace('`', "``"))
}

fn map(canonical: &str) -> String {
    match canonical {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" | "datetime" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "binary" => "BLOB",
        _ => panic!("Unknown mysql type \"{canonical}\""),
    }
    .into()
}

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "mysql",
        quote: quote,
        create_table: |table, columns| {
            let mapped: Vec<Column> = columns
                .iter()
                .map(|column| {
                    let affinity = map(&column.type_name);
                    Column::new(
                        quote(&column.name),
                        if column.id {
                            format!("{affinity} PRIMARY KEY AUTO_INCREMENT")
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

fn replace_db(url: &str, name: &str) -> String {
    match url.rfind('/') {
        Some(index) if url[..index].contains("://") => {
            let prefix = &url[..=index];
            match url[index + 1..].find('?') {
                Some(q) => format!("{prefix}{name}{}", &url[index + 1 + q..]),
                None => format!("{prefix}{name}"),
            }
        }
        _ => url.to_string(),
    }
}

#[tokio::test]
async fn user_crud() {
    let Ok(base) = std::env::var("MYSQL_URL") else {
        return;
    };
    for path in datasources_sql_specs::files() {
        let name = format!(
            "ds_{:x}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let mut admin = Conn::from_url(base.clone()).await.unwrap();
        admin
            .query_drop(format!("CREATE DATABASE `{name}`"))
            .await
            .unwrap();
        drop(admin);
        let url = replace_db(&base, &name);
        let conn = Arc::new(MysqlConnection::new(Some(url)));
        conn.open().await.unwrap();
        let command = Arc::new(MysqlCommand::new(conn.clone()));
        datasources_sql_specs::run(&path, &dialect(), conn, command)
            .await
            .unwrap();
        let mut admin = Conn::from_url(base.clone()).await.unwrap();
        admin
            .query_drop(format!("DROP DATABASE `{name}`"))
            .await
            .ok();
    }
}
