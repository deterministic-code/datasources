use chrono::{DateTime, TimeZone, Utc};
use datasources::{
    BatchCommandIBulkWriteStrategy, BulkCRUDRepository, BulkWriteStrategy, CRUDRepositoryOptions,
    Driver, Native, Row, RowTypeConverter, StoredProcedureBulkWriteStrategy, Value,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use user_roles_simple_cli::{migrate, schema};

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
}

async fn run() -> datasources::Result<()> {
    let provider = env("PROVIDER", "sqlite");
    let strategy_name = env("PERF_STRATEGY", "batch");
    let rows = env("PERF_ROWS", "1000").parse::<usize>().expect("PERF_ROWS");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("PERF_CHUNK");
    let url = env("DATASOURCE_URL", "sqlite:///:memory:");
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), url);
    let connection = driver(&provider).connection(Some(&options));
    connection.open().await?;
    let result = work(
        Arc::clone(&connection),
        &provider,
        &strategy_name,
        rows,
        chunk,
    )
    .await;
    let _ = connection.close().await;
    result
}

async fn work(
    connection: Arc<dyn datasources::Connection>,
    provider: &str,
    strategy_name: &str,
    rows: usize,
    chunk: usize,
) -> datasources::Result<()> {
    let command = connection.create_command();
    migrate::apply(command.as_ref(), provider).await?;
    if provider == "mysql" || provider == "postgres" {
        let shared = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../user-roles-simple-cli-shared");
        let sql = std::fs::read_to_string(shared.join(format!("bulk-{provider}.sql")))
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        for part in sql.split("---").map(str::trim).filter(|part| !part.is_empty()) {
            command
                .execute_native(&Native::new(part), None)
                .await?;
        }
    }
    let batched = batched(provider, strategy_name);
    let strategy: Arc<dyn BulkWriteStrategy> = match (strategy_name, batched) {
        ("single", _) => Arc::new(datasources::SingleCommandIBulkWriteStrategy::new()),
        (_, true) => Arc::new(BatchCommandIBulkWriteStrategy::new()),
        ("procedure", _) => Arc::new(StoredProcedureBulkWriteStrategy::new(
            "bulk_create_user",
            "bulk_update_user",
            "bulk_delete_user",
        )),
        (name, _) => panic!("{name}"),
    };
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(RowTypeConverter::new(vec![
        "id".into(),
        "username".into(),
        "password".into(),
        "is_enabled".into(),
        "created".into(),
        "updated".into(),
    ]));
    let users_repo = BulkCRUDRepository::with_strategy(Arc::clone(&connection), options, strategy)?;
    let mut users = load_users(provider, rows)?;
    let create_started = Instant::now();
    create_chunks(&users_repo, &mut users, batched, chunk).await?;
    let create_ms = create_started.elapsed().as_secs_f64() * 1000.0;
    if count(command.as_ref(), "users").await? != users.len() as i64 {
        return Err(datasources::DatasourceError::message("create count"));
    }
    for user in &mut users {
        user.insert("is_enabled".into(), schema::flag(provider, false));
    }
    let update_started = Instant::now();
    update_chunks(&users_repo, &users, batched, chunk).await?;
    let update_ms = update_started.elapsed().as_secs_f64() * 1000.0;
    println!(
        "{}",
        serde_json::json!({
            "sample": "user-roles-simple-cli",
            "language": "rust",
            "provider": provider,
            "strategy": strategy_name,
            "rows": users.len(),
            "chunk": if batched { chunk.min(users.len()) } else { users.len() },
            "createMs": create_ms,
            "updateMs": update_ms,
            "createPerSec": users.len() as f64 / (create_ms / 1000.0),
            "updatePerSec": users.len() as f64 / (update_ms / 1000.0),
            "counts": { "users": users.len() },
        })
    );
    Ok(())
}

fn load_users(provider: &str, rows: usize) -> datasources::Result<Vec<Row>> {
    if let Ok(path) = std::env::var("PERF_DATA") {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        let items: Vec<serde_json::Value> = serde_json::from_str(&text)
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        return items.iter().map(|item| user_from_json(provider, item)).collect();
    }
    let created = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    Ok((1..=rows as i64)
        .map(|id| {
            row(&[
                ("id", Value::Int(id)),
                ("username", Value::from(format!("user{id}"))),
                ("password", Value::from("hash")),
                ("is_enabled", schema::flag(provider, true)),
                ("created", created.into()),
                ("updated", Value::from(vec![1_u8, 2, 3])),
            ])
        })
        .collect())
}

fn user_from_json(provider: &str, item: &serde_json::Value) -> datasources::Result<Row> {
    let id = item["id"].as_i64().unwrap_or(0);
    let enabled = item["is_enabled"].as_bool().unwrap_or(true);
    let created = item["created"]
        .as_str()
        .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
        .map(|stamp| stamp.with_timezone(&Utc))
        .unwrap_or_else(|| Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
    let updated = item["updated"]
        .as_array()
        .map(|bytes| bytes.iter().filter_map(|byte| byte.as_u64().map(|n| n as u8)).collect())
        .unwrap_or_else(|| vec![1_u8, 2, 3]);
    Ok(row(&[
        ("id", Value::Int(id)),
        (
            "username",
            Value::from(item["username"].as_str().unwrap_or("").to_string()),
        ),
        (
            "password",
            Value::from(item["password"].as_str().unwrap_or("").to_string()),
        ),
        ("is_enabled", schema::flag(provider, enabled)),
        ("created", created.into()),
        ("updated", Value::from(updated)),
    ]))
}

fn batched(provider: &str, strategy_name: &str) -> bool {
    strategy_name == "batch" || strategy_name == "procedure" && provider == "sqlite"
}

async fn create_chunks(
    repo: &BulkCRUDRepository,
    items: &mut [Row],
    batched: bool,
    chunk: usize,
) -> datasources::Result<()> {
    if !batched {
        repo.create_bulk(items).await?;
        return Ok(());
    }
    for batch in items.chunks_mut(chunk) {
        repo.create_bulk(batch).await?;
    }
    Ok(())
}

async fn update_chunks(
    repo: &BulkCRUDRepository,
    items: &[Row],
    batched: bool,
    chunk: usize,
) -> datasources::Result<()> {
    if !batched {
        repo.update_bulk(items, None).await?;
        return Ok(());
    }
    for batch in items.chunks(chunk) {
        repo.update_bulk(batch, None).await?;
    }
    Ok(())
}

async fn count(command: &dyn datasources::Command, table: &str) -> datasources::Result<i64> {
    let reader = command
        .execute_reader_native(&Native::new(format!("SELECT COUNT(*) AS n FROM {table}")), None)
        .await?;
    reader.next().await?;
    match reader.current().as_ref().and_then(|row| row.get("n")) {
        Some(Value::Int(n)) => Ok(*n),
        Some(value) => Ok(value.wire_text().parse().unwrap_or(0)),
        None => Ok(0),
    }
}

fn driver(provider: &str) -> Box<dyn Driver> {
    match provider {
        "sqlite" => Box::new(datasources_sqlite::SqliteDriver),
        "mysql" => Box::new(datasources_mysql::MysqlDriver),
        "postgres" => Box::new(datasources_postgres::PostgresDriver),
        name => panic!("{name}"),
    }
}

fn row(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn env(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.into())
}
