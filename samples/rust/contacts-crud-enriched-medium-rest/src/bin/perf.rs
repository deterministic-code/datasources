use chrono::{DateTime, TimeZone, Utc};
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
    let batched = batched(provider, strategy_name);
    let settings_repo = Arc::new(repo(
        Arc::clone(&connection),
        strategy_name,
        batched,
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
        batched,
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
        batched,
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
        batched,
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
    let mut contacts = load_contacts(rows, now)?;
    let create_started = Instant::now();
    create_chunks(&contacts_repo, &mut contacts, batched, chunk).await?;
    let create_ms = create_started.elapsed().as_secs_f64() * 1000.0;
    let counts = serde_json::json!({
        "contacts": count(command.as_ref(), "contacts").await?,
        "addresses": count(command.as_ref(), "addresses").await?,
        "phones": count(command.as_ref(), "phones").await?,
        "settings": count(command.as_ref(), "contact_settings").await?,
    });
    let each = related_counts(&contacts);
    if counts["contacts"] != contacts.len() as i64
        || counts["addresses"] != (contacts.len() * each.0) as i64
        || counts["phones"] != (contacts.len() * each.1) as i64
        || counts["settings"] != (contacts.len() * each.2) as i64
    {
        return Err(datasources::DatasourceError::message(format!(
            "create count {counts}"
        )));
    }
    for contact in &mut contacts {
        contact.insert("notes".into(), Value::from("updated"));
        contact.insert(
            "addresses".into(),
            encode_rows(&set_field(decode_rows(contact.get("addresses")), "city", "City")),
        );
        contact.insert(
            "phones".into(),
            encode_rows(&set_field(decode_rows(contact.get("phones")), "label", "w")),
        );
        contact.insert(
            "settings".into(),
            encode_rows(&set_field(decode_rows(contact.get("settings")), "value", "dark")),
        );
    }
    let update_started = Instant::now();
    update_chunks(&contacts_repo, &contacts, batched, chunk).await?;
    let update_ms = update_started.elapsed().as_secs_f64() * 1000.0;
    println!(
        "{}",
        serde_json::json!({
            "sample": "contacts-crud-enriched-medium-rest",
            "language": "rust",
            "provider": provider,
            "strategy": strategy_name,
            "rows": contacts.len(),
            "chunk": if batched { chunk.min(contacts.len()) } else { contacts.len() },
            "createMs": create_ms,
            "updateMs": update_ms,
            "createPerSec": contacts.len() as f64 / (create_ms / 1000.0),
            "updatePerSec": contacts.len() as f64 / (update_ms / 1000.0),
            "counts": counts,
        })
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn load_contacts(rows: usize, now: DateTime<Utc>) -> datasources::Result<Vec<Row>> {
    if let Ok(path) = std::env::var("PERF_DATA") {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        let items: Vec<serde_json::Value> = serde_json::from_str(&text)
            .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
        return items.iter().map(contact_from_json).collect();
    }
    let each = env("PERF_RELATED", "10").parse::<i64>().unwrap_or(10);
    Ok((1..=rows as i64)
        .map(|id| {
            let children = |prefix: &str| {
                (0..each)
                    .map(|n| {
                        let child = (id - 1) * each + n + 1;
                        let mut pairs = vec![
                            ("id", Value::Int(child)),
                            ("uuid", Value::from(format!("{prefix}{child}"))),
                            ("created", now.into()),
                            ("updated", now.into()),
                            ("version", Value::from(vec![1_u8])),
                        ];
                        if prefix == "a" {
                            pairs.extend([
                                ("addressTypeId", Value::Int(1)),
                                ("line1", Value::from("1 Main")),
                                ("city", Value::from("Town")),
                            ]);
                        } else {
                            pairs.extend([
                                ("phoneTypeId", Value::Int(1)),
                                ("number", Value::from("5550000")),
                                ("label", Value::from("m")),
                            ]);
                        }
                        row(&pairs)
                    })
                    .collect::<Vec<_>>()
            };
            row(&[
                ("id", Value::Int(id)),
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
                ("addresses", encode_rows(&children("a"))),
                ("phones", encode_rows(&children("p"))),
                (
                    "settings",
                    encode_rows(
                        &(0..each)
                            .map(|n| {
                                row(&[
                                    ("key", Value::from(format!("k{n}"))),
                                    ("value", Value::from("light")),
                                ])
                            })
                            .collect::<Vec<_>>(),
                    ),
                ),
            ])
        })
        .collect())
}

fn contact_from_json(item: &serde_json::Value) -> datasources::Result<Row> {
    let id = json_i64(&item["id"]);
    let stamp = json_stamp(&item["created"]);
    let settings = item["settings"]
        .as_object()
        .map(|map| {
            map.iter()
                .map(|(key, value)| {
                    row(&[
                        ("key", Value::from(key.clone())),
                        (
                            "value",
                            Value::from(value.as_str().unwrap_or("").to_string()),
                        ),
                    ])
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(row(&[
        ("id", Value::Int(id)),
        (
            "uuid",
            Value::from(item["uuid"].as_str().unwrap_or("").to_string()),
        ),
        ("created", stamp.into()),
        ("updated", json_stamp(&item["updated"]).into()),
        ("version", Value::from(json_bytes(&item["version"]))),
        ("contactSourceId", Value::Int(json_i64(&item["contactSourceId"]))),
        (
            "firstName",
            Value::from(item["firstName"].as_str().unwrap_or("").to_string()),
        ),
        (
            "lastName",
            Value::from(item["lastName"].as_str().unwrap_or("").to_string()),
        ),
        (
            "email",
            Value::from(item["email"].as_str().unwrap_or("").to_string()),
        ),
        (
            "notes",
            Value::from(item["notes"].as_str().unwrap_or("").to_string()),
        ),
        (
            "addresses",
            encode_rows(&json_children(&item["addresses"])),
        ),
        ("phones", encode_rows(&json_children(&item["phones"]))),
        ("settings", encode_rows(&settings)),
    ]))
}

fn json_children(value: &serde_json::Value) -> Vec<Row> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| {
            let mut pairs = vec![
                ("id", Value::Int(json_i64(&item["id"]))),
                (
                    "uuid",
                    Value::from(item["uuid"].as_str().unwrap_or("").to_string()),
                ),
                ("created", json_stamp(&item["created"]).into()),
                ("updated", json_stamp(&item["updated"]).into()),
                ("version", Value::from(json_bytes(&item["version"]))),
            ];
            if item.get("addressTypeId").is_some() {
                pairs.push(("addressTypeId", Value::Int(json_i64(&item["addressTypeId"]))));
                pairs.push((
                    "line1",
                    Value::from(item["line1"].as_str().unwrap_or("").to_string()),
                ));
                pairs.push((
                    "city",
                    Value::from(item["city"].as_str().unwrap_or("").to_string()),
                ));
            }
            if item.get("phoneTypeId").is_some() {
                pairs.push(("phoneTypeId", Value::Int(json_i64(&item["phoneTypeId"]))));
                pairs.push((
                    "number",
                    Value::from(item["number"].as_str().unwrap_or("").to_string()),
                ));
                pairs.push((
                    "label",
                    Value::from(item["label"].as_str().unwrap_or("").to_string()),
                ));
            }
            row(&pairs)
        })
        .collect()
}

fn json_i64(value: &serde_json::Value) -> i64 {
    value.as_i64().or_else(|| value.as_u64().map(|n| n as i64)).unwrap_or(0)
}

fn json_bytes(value: &serde_json::Value) -> Vec<u8> {
    value
        .as_array()
        .map(|bytes| bytes.iter().filter_map(|byte| byte.as_u64().map(|n| n as u8)).collect())
        .unwrap_or_else(|| vec![1_u8])
}

fn json_stamp(value: &serde_json::Value) -> DateTime<Utc> {
    value
        .as_str()
        .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
        .map(|stamp| stamp.with_timezone(&Utc))
        .unwrap_or_else(|| Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap())
}

fn related_counts(contacts: &[Row]) -> (usize, usize, usize) {
    let first = contacts.first();
    (
        decode_rows(first.and_then(|contact| contact.get("addresses"))).len(),
        decode_rows(first.and_then(|contact| contact.get("phones"))).len(),
        decode_rows(first.and_then(|contact| contact.get("settings"))).len(),
    )
}

fn decode_rows(value: Option<&Value>) -> Vec<Row> {
    let Some(Value::Text(text)) = value else {
        return vec![];
    };
    text.split('|')
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.split(',')
                .filter_map(|cell| {
                    let (key, raw) = cell.split_once('=')?;
                    Some((
                        key.to_string(),
                        Value::from(
                            raw.replace("%3D", "=")
                                .replace("%2C", ",")
                                .replace("%7C", "|")
                                .replace("%25", "%"),
                        ),
                    ))
                })
                .collect()
        })
        .collect()
}

fn set_field(mut items: Vec<Row>, field: &str, value: &str) -> Vec<Row> {
    for item in &mut items {
        item.insert(field.into(), Value::from(value.to_string()));
    }
    items
}

fn batched(provider: &str, strategy_name: &str) -> bool {
    strategy_name == "batch" || strategy_name == "procedure" && provider == "sqlite"
}

fn repo(
    connection: Arc<dyn datasources::Connection>,
    strategy_name: &str,
    batched: bool,
    entity: &str,
    entities: &[(&str, &str)],
    properties: &[(&str, &str)],
    columns: &[&str],
    ids: &[&str],
    create: &str,
    update: &str,
    delete: &str,
) -> datasources::Result<BulkCRUDRepository> {
    let strategy: Arc<dyn BulkWriteStrategy> = match (strategy_name, batched) {
        ("single", _) => Arc::new(datasources::SingleCommandIBulkWriteStrategy::new()),
        (_, true) => Arc::new(BatchCommandIBulkWriteStrategy::new()),
        ("procedure", _) => Arc::new(StoredProcedureBulkWriteStrategy::new(create, update, delete)),
        (name, _) => panic!("{name}"),
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
