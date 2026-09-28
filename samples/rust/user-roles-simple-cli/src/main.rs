use datasources::{
    provider_from_url, utc_iso_millis, ConsoleCommandMiddleware, DatasourceError, Driver,
};
use datasources_mysql::MysqlDriver;
use datasources_postgres::PostgresDriver;
use datasources_sqlite::SqliteDriver;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
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
    let url = env("DATASOURCE_URL")
        .ok_or_else(|| DatasourceError::message("DATASOURCE_URL is required"))?;
    let schema = env("DATABASE_SCHEMA");
    let provider =
        provider_from_url(&url).ok_or_else(|| DatasourceError::message("unknown provider"))?;
    let options = |schema: Option<&str>| {
        let mut map = HashMap::new();
        map.insert("url".to_string(), url.clone());
        if let Some(schema) = schema {
            map.insert("schema".to_string(), schema.to_string());
        }
        map
    };
    let driver: Arc<dyn Driver> = match provider.as_str() {
        "sqlite" => Arc::new(SqliteDriver),
        "mysql" => Arc::new(MysqlDriver),
        "postgres" => Arc::new(PostgresDriver),
        name => {
            return Err(DatasourceError::message(format!("unknown provider {name}")));
        }
    };
    let connection = match provider.as_str() {
        "postgres" => driver.connection(Some(&options(schema.as_deref()))),
        _ => driver.connection(Some(&options(None))),
    };
    connection.open().await?;
    let command = connection.create_command();
    migrate::apply(command.as_ref(), connection.provider(), driver.as_ref()).await?;
    let stamp = env("CLI_NOW");
    let clock = stamp.clone();
    command.add_middleware(Arc::new(ConsoleCommandMiddleware::with_sink(
        |line| eprintln!("{line}"),
        move || {
            clock
                .clone()
                .unwrap_or_else(|| utc_iso_millis(SystemTime::now()))
        },
    )));
    let app = App::new(command.as_ref(), connection.provider());
    let app = match stamp {
        Some(stamp) => app.with_now(move || stamp.clone()),
        None => app,
    };
    print!("{}", app.run(parsed).await?);
    Ok(())
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
