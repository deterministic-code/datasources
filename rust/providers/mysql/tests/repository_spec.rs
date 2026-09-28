use super::command::MysqlCommand;
use super::connection::MysqlConnection;
use datasources::Connection;
use datasources_sql_specs::RepositoryDialect;
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::sync::Arc;
use std::time::SystemTime;

fn quote(id: &str) -> String {
    format!("`{}`", id.replace('`', "``"))
}

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "mysql",
        quote,
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
