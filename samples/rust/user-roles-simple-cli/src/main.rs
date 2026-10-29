use datasources::{utc_iso_millis, ConsoleCommandMiddleware, DatasourceError};
use std::sync::Arc;
use std::time::SystemTime;
use user_roles_simple_cli::app::App;
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
    datasources::register("sqlite", Arc::new(datasources_sqlite::SqliteDriver));
    datasources::register("mysql", Arc::new(datasources_mysql::MysqlDriver));
    datasources::register("postgres", Arc::new(datasources_postgres::PostgresDriver));
    let provider = env("PROVIDER").unwrap_or_else(|| "sqlite".into());
    let url = env("DATASOURCE_URL").unwrap_or_else(|| "sqlite:///./app.sqlite".into());
    let root = env("MIGRATIONS_ROOT").unwrap_or_else(|| "sql".into());
    migrate_up(&provider, &url, &root);
    let ds = datasources::default()?;
    let connection = ds.create_connection().await?;
    connection.open().await?;
    let command = ds.create_command(Some(connection)).await?;
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
    let app = App::new(command.as_ref(), ds.provider());
    let app = match stamp {
        Some(stamp) => app.with_now(move || stamp.clone()),
        None => app,
    };
    print!("{}", app.run(parsed).await?);
    ds.close().await?;
    Ok(())
}

fn migrate_up(provider: &str, url: &str, root: &str) {
    let output = std::process::Command::new("node")
        .arg(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../..")
                .join("scripts/migraters.mjs"),
        )
        .args(["up", "--provider", provider, "--connection", url, "--migrations-root", root])
        .output()
        .expect("migrate-up");
    if !output.status.success() {
        panic!(
            "migrate-up\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
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
