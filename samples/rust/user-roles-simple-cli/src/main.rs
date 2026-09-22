use datasources::{provider_from_url, Connection, DatasourceError};
use datasources_mysql::{MysqlAsyncDriver, MysqlConnection};
use datasources_postgres::{PostgresConnection, TokioPostgresDriver};
use datasources_sqlite::{RusqliteDriver, SqliteConnection};
use std::sync::Arc;
use user_roles_simple_cli::app::App;
use user_roles_simple_cli::migrate;
use user_roles_simple_cli::parse;

#[tokio::main]
async fn main() {
    match run(std::env::args().skip(1).collect()).await {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

async fn run(args: Vec<String>) -> Result<(), DatasourceError> {
    let parsed = parse::args(&args)?;
    let url = env("DATASOURCE_URL").ok_or_else(|| DatasourceError::message("DATASOURCE_URL is required"))?;
    let schema = env("DATABASE_SCHEMA");
    let provider = provider_from_url(&url)
        .ok_or_else(|| DatasourceError::message("unknown provider"))?;
    let connection: Arc<dyn Connection> = match provider.as_str() {
        "sqlite" => Arc::new(SqliteConnection::new(
            Some(sqlite_file(&url)),
            Arc::new(RusqliteDriver::new()),
        )),
        "mysql" => Arc::new(MysqlConnection::new(
            Some(url.clone()),
            Arc::new(MysqlAsyncDriver::new()),
        )),
        "postgres" => Arc::new(PostgresConnection::new(
            Some(url.clone()),
            schema.clone(),
            Arc::new(TokioPostgresDriver::new(schema)),
        )),
        name => {
            return Err(DatasourceError::message(format!("unknown provider {name}")));
        }
    };
    connection.open().await?;
    let command = connection.create_command();
    migrate::apply(command.as_ref(), connection.provider()).await?;
    let app = App::new(command.as_ref(), connection.provider());
    let app = match env("CLI_NOW") {
        Some(stamp) => app.with_now(move || stamp.clone()),
        None => app,
    };
    print!("{}", app.run(parsed).await?);
    Ok(())
}

fn sqlite_file(url: &str) -> String {
    url.strip_prefix("sqlite:///")
        .or_else(|| url.strip_prefix("sqlite://"))
        .or_else(|| url.strip_prefix("sqlite:"))
        .unwrap_or(":memory:")
        .to_string()
}

fn env(key: &str) -> Option<String> {
    if let Ok(value) = std::env::var(key) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    let text = std::fs::read_to_string(".env").ok()?;
    for line in text.lines() {
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if name.trim() == key {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}
