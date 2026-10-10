use chrono::{TimeZone, Utc};
use datasources::{
    encode_rows, BatchCommandIBulkWriteStrategy, BulkCRUDRepository, BulkWriteStrategy,
    CRUDRepositoryOptions, DictionaryMapper, Driver, Enrichment, EnrichmentPropertyType, Native,
    Row, RowTypeConverter, StoredProcedureBulkWriteStrategy, Value,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

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
    let result = work(Arc::clone(&connection), &provider, &strategy_name, rows, chunk).await;
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
    let shared = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../contacts-crud-enriched-medium-rest-shared");
    let schema = std::fs::read_to_string(shared.join(format!("schema-{provider}.sql")))
        .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
    for part in schema.split(';').map(str::trim).filter(|part| !part.is_empty()) {
        command
            .execute_native(&Native::new(part), None)
            .await?;
    }
    if provider == "mysql" || provider == "postgres" {
        let sql = std::fs::read_to_string(shared.join(format!("bulk-{provider}.sql")))
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        for part in sql.split("---").map(str::trim).filter(|part| !part.is_empty()) {
            command
                .execute_native(&Native::new(part), None)
                .await?;
        }
    }
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let stamp = if provider == "mysql" {
        "2024-01-01 00:00:00".to_string()
    } else {
        now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    };
    let blob = match provider {
        "mysql" => "0x01",
        "postgres" => "'\\x01'",
        _ => "x'01'",
    };
    for table in ["contact_sources", "address_types", "phone_types"] {
        command
            .execute_native(
                &Native::new(format!(
                    "INSERT INTO {table} (id, uuid, created, updated, version, name) VALUES (1, '00000000-0000-0000-0000-000000000001', '{stamp}', '{stamp}', {blob}, 'seed')"
                )),
                None,
            )
            .await?;
    }
    let settings_repo = Arc::new(repo(
        Arc::clone(&connection),
        strategy_name,
        "ContactSettings",
        &[("ContactSettings", "contact_settings")],
        &[("contactId", "contact_id")],
        &["contactId", "key", "value"],
        &["contactId", "key"],
        "bulk_create_setting",
        "bulk_update_setting",
        "bulk_delete_setting",
    )?);
    let addresses_repo = Arc::new(repo(
        Arc::clone(&connection),
        strategy_name,
        "Address",
        &[("Address", "addresses")],
        &[("contactId", "contact_id"), ("addressTypeId", "address_type_id")],
        &[
            "id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1",
            "city",
        ],
        &["id"],
        "bulk_create_address",
        "bulk_update_address",
        "bulk_delete_address",
    )?);
    let phones_repo = Arc::new(repo(
        Arc::clone(&connection),
        strategy_name,
        "Phone",
        &[("Phone", "phones")],
        &[("contactId", "contact_id"), ("phoneTypeId", "phone_type_id")],
        &[
            "id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number",
            "label",
        ],
        &["id"],
        "bulk_create_phone",
        "bulk_update_phone",
        "bulk_delete_phone",
    )?);
    let mut contacts_repo = repo(
        Arc::clone(&connection),
        strategy_name,
        "Contact",
        &[("Contact", "contacts")],
        &[
            ("contactSourceId", "contact_source_id"),
            ("firstName", "first_name"),
            ("lastName", "last_name"),
        ],
        &[
            "id",
            "uuid",
            "created",
            "updated",
            "version",
            "contactSourceId",
            "firstName",
            "lastName",
            "email",
            "notes",
        ],
        &["id"],
        "bulk_create_contact",
        "bulk_update_contact",
        "bulk_delete_contact",
    )?;
    contacts_repo.add_enriched(
        Enrichment::new("addresses", "id", "contactId", None, EnrichmentPropertyType::List),
        Arc::clone(&addresses_repo),
        None,
    );
    contacts_repo.add_enriched(
        Enrichment::new("phones", "id", "contactId", None, EnrichmentPropertyType::List),
        Arc::clone(&phones_repo),
        None,
    );
    contacts_repo.add_enriched(
        Enrichment::new(
            "settings",
            "id",
            "contactId",
            Some("value".into()),
            EnrichmentPropertyType::Dictionary,
        ),
        settings_repo,
        None,
    );
    let mut contacts: Vec<Row> = (1..=rows as i64)
        .map(|id| {
            let keyed = |pairs: &[(&str, Value)]| {
                let mut item = row(pairs);
                if strategy_name == "batch" {
                    item.insert("id".into(), Value::Int(id));
                }
                item
            };
            keyed(&[
                (
                    "uuid",
                    Value::from(format!("00000000-0000-0000-0000-{id:012x}")),
                ),
                ("created", now.into()),
                ("updated", now.into()),
                ("version", Value::from(vec![1_u8])),
                ("contactSourceId", Value::Int(1)),
                ("firstName", Value::from(format!("First{id}"))),
                ("lastName", Value::from(format!("Last{id}"))),
                ("email", Value::from(format!("c{id}@ex.test"))),
                ("notes", Value::from("")),
                (
                    "addresses",
                    encode_rows(&[keyed(&[
                        ("uuid", Value::from(format!("a{id}"))),
                        ("created", now.into()),
                        ("updated", now.into()),
                        ("version", Value::from(vec![1_u8])),
                        ("addressTypeId", Value::Int(1)),
                        ("line1", Value::from("1 Main")),
                        ("city", Value::from("Town")),
                    ])]),
                ),
                (
                    "phones",
                    encode_rows(&[keyed(&[
                        ("uuid", Value::from(format!("p{id}"))),
                        ("created", now.into()),
                        ("updated", now.into()),
                        ("version", Value::from(vec![1_u8])),
                        ("phoneTypeId", Value::Int(1)),
                        ("number", Value::from("5550000")),
                        ("label", Value::from("m")),
                    ])]),
                ),
                (
                    "settings",
                    encode_rows(&[row(&[
                        ("key", Value::from("theme")),
                        ("value", Value::from("light")),
                    ])]),
                ),
            ])
        })
        .collect();
    let create_started = Instant::now();
    create_chunks(&contacts_repo, &mut contacts, strategy_name, chunk).await?;
    let create_ms = create_started.elapsed().as_secs_f64() * 1000.0;
    let counts = serde_json::json!({
        "contacts": count(command.as_ref(), "contacts").await?,
        "addresses": count(command.as_ref(), "addresses").await?,
        "phones": count(command.as_ref(), "phones").await?,
        "settings": count(command.as_ref(), "contact_settings").await?,
    });
    if counts["contacts"] != rows as i64
        || counts["addresses"] != rows as i64
        || counts["phones"] != rows as i64
        || counts["settings"] != rows as i64
    {
        return Err(datasources::DatasourceError::message(format!(
            "create count {counts}"
        )));
    }
    for contact in &mut contacts {
        contact.insert("notes".into(), Value::from("updated"));
        contact.insert(
            "addresses".into(),
            encode_rows(&[row(&[
                ("id", contact.get("id").cloned().unwrap_or(Value::Null)),
                (
                    "uuid",
                    Value::from(format!(
                        "a{}",
                        contact.get("id").map(Value::wire_text).unwrap_or_default()
                    )),
                ),
                ("created", now.into()),
                ("updated", now.into()),
                ("version", Value::from(vec![1_u8])),
                ("addressTypeId", Value::Int(1)),
                ("line1", Value::from("1 Main")),
                ("city", Value::from("City")),
            ])]),
        );
        contact.insert(
            "phones".into(),
            encode_rows(&[row(&[
                ("id", contact.get("id").cloned().unwrap_or(Value::Null)),
                (
                    "uuid",
                    Value::from(format!(
                        "p{}",
                        contact.get("id").map(Value::wire_text).unwrap_or_default()
                    )),
                ),
                ("created", now.into()),
                ("updated", now.into()),
                ("version", Value::from(vec![1_u8])),
                ("phoneTypeId", Value::Int(1)),
                ("number", Value::from("5550000")),
                ("label", Value::from("w")),
            ])]),
        );
        contact.insert(
            "settings".into(),
            encode_rows(&[row(&[
                ("key", Value::from("theme")),
                ("value", Value::from("dark")),
            ])]),
        );
    }
    let update_started = Instant::now();
    update_chunks(&contacts_repo, &contacts, strategy_name, chunk).await?;
    let update_ms = update_started.elapsed().as_secs_f64() * 1000.0;
    println!(
        "{}",
        serde_json::json!({
            "sample": "contacts-crud-enriched-medium-rest",
            "language": "rust",
            "provider": provider,
            "strategy": strategy_name,
            "rows": rows,
            "chunk": if strategy_name == "batch" { chunk } else { rows },
            "createMs": create_ms,
            "updateMs": update_ms,
            "createPerSec": rows as f64 / (create_ms / 1000.0),
            "updatePerSec": rows as f64 / (update_ms / 1000.0),
            "counts": counts,
        })
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn repo(
    connection: Arc<dyn datasources::Connection>,
    strategy_name: &str,
    entity: &str,
    entities: &[(&str, &str)],
    properties: &[(&str, &str)],
    columns: &[&str],
    ids: &[&str],
    create: &str,
    update: &str,
    delete: &str,
) -> datasources::Result<BulkCRUDRepository> {
    let strategy: Arc<dyn BulkWriteStrategy> = match strategy_name {
        "single" => Arc::new(datasources::SingleCommandIBulkWriteStrategy::new()),
        "batch" => Arc::new(BatchCommandIBulkWriteStrategy::new()),
        "procedure" => Arc::new(StoredProcedureBulkWriteStrategy::new(create, update, delete)),
        name => panic!("{name}"),
    };
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = ids.iter().map(|id| (*id).to_string()).collect();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new(
        entities
            .iter()
            .map(|(from, to)| (from.to_string(), to.to_string())),
    )));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new(
        properties
            .iter()
            .map(|(from, to)| (from.to_string(), to.to_string())),
    )));
    options.row_converter = Some(RowTypeConverter::new(
        columns.iter().map(|name| (*name).to_string()).collect(),
    ));
    BulkCRUDRepository::with_strategy(connection, options, strategy)
}

async fn create_chunks(
    repo: &BulkCRUDRepository,
    items: &mut [Row],
    strategy_name: &str,
    chunk: usize,
) -> datasources::Result<()> {
    if strategy_name != "batch" {
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
    strategy_name: &str,
    chunk: usize,
) -> datasources::Result<()> {
    if strategy_name != "batch" {
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
