mod datasources_providers;
use datasources::{utc_iso_millis, ConsoleCommandMiddleware, DatasourceError};
use std::sync::Arc;
use std::time::SystemTime;
use user_roles_simple_cli::program;

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
    datasources_providers::register();
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
    print!(
        "{}",
        program::run(command.as_ref(), &args, program::hash, true).await?
    );
    ds.close().await?;
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
