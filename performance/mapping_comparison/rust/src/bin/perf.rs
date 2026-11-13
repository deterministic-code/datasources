use chrono::{DateTime, Utc};
use datasources::{
    encode_rows, default, BatchCommandIBulkWriteStrategy, BulkCRUDRepository,
    CRUDRepositoryOptions, ChangeCasingMapper, Connection, Casing, DictionaryMapper, Enrichment,
    EnrichmentPropertyType, IReadonlyRepository, Mapper, Native, ReadonlyRepository,
    ReadonlyRepositoryMappingOptions, Result, Row, RowTypeConverter, Schema, SchemaColumn,
    StoredProcedureBulkWriteStrategy, Value,
};
use futures_util::TryStreamExt;
use mapping_comparison::datasources_providers;
use mapping_comparison::generated::{Address, Contact, Email};
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    datasources_providers::register();
    let provider = env("PROVIDER", "sqlite");
    let rows = env("PERF_ROWS", "25000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let read_iters = env("PERF_READ_ITERS", "3").parse::<usize>().expect("iters");
    let batched = provider == "sqlite";

    let ds = default()?;
    let connection = ds.create_connection().await?;
    connection.open().await?;

    let contact_columns = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "name",
        "given_name",
        "additional_name",
        "pronunciation",
        "family_name",
        "nick_name",
        "job_title",
        "organization",
        "is_starred",
        "is_favorite",
        "is_deleted",
        "gender_type_id",
    ];
    let address_columns = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_id",
        "display_name",
        "line_1",
        "line_2",
        "line_3",
        "neighborhood",
        "district",
        "city",
        "state_province",
        "postal_code",
        "country_code",
        "formatted_address",
        "latitude",
        "longitude",
        "is_primary",
    ];
    let email_columns = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "email_type_id",
        "contact_id",
        "address",
        "display_name",
        "is_primary",
    ];

    let addresses = Arc::new(repo(
        Arc::clone(&connection),
        batched,
        "Address",
        "addresses",
        &[],
        &address_columns,
        &["id"],
        "bulk_create_addresses",
        "bulk_update_addresses",
        "bulk_delete_addresses",
    )?);
    let emails = Arc::new(repo(
        Arc::clone(&connection),
        batched,
        "Email",
        "emails",
        &[],
        &email_columns,
        &["id"],
        "bulk_create_emails",
        "bulk_update_emails",
        "bulk_delete_emails",
    )?);
    let mut contacts_bulk = repo(
        Arc::clone(&connection),
        batched,
        "Contact",
        "contacts",
        &[],
        &contact_columns,
        &["id"],
        "bulk_create_contacts",
        "bulk_update_contacts",
        "bulk_delete_contacts",
    )?;
    contacts_bulk.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&addresses),
        None,
    );
    contacts_bulk.add_enriched(
        Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&emails),
        None,
    );
    let contacts_bulk = Arc::new(contacts_bulk);

    let mut seeded = seed_contacts(rows);
    let expected_addresses = seeded
        .iter()
        .map(|row| related_len(row, "addresses"))
        .sum::<usize>();
    let expected_emails = seeded
        .iter()
        .map(|row| related_len(row, "emails"))
        .sum::<usize>();
    let create_started = Instant::now();
    each(&mut seeded, chunk, |batch| {
        let repo = Arc::clone(&contacts_bulk);
        async move { repo.create_bulk(batch).await.map(|_| ()) }
    })
    .await?;
    let create_ms = create_started.elapsed().as_secs_f64() * 1000.0;
    let counts = json!({
        "contacts": count(&connection, "contacts").await?,
        "addresses": count(&connection, "addresses").await?,
        "emails": count(&connection, "emails").await?,
    });
    if counts["contacts"] != rows
        || counts["addresses"] != expected_addresses as i64
        || counts["emails"] != expected_emails as i64
    {
        return Err(datasources::DatasourceError::message(format!(
            "create count {counts} expected addresses={expected_addresses} emails={expected_emails}"
        )));
    }

    let default_repo = ds.contacts();
    let snake = ChangeCasingMapper::new(Casing::Snake, Casing::Snake);
    let dictionary_addresses = Arc::new(ReadonlyRepository::<Address>::new(
        Arc::clone(&connection),
        dictionary_options("Address", "addresses", &address_columns, &snake)?,
    )?);
    let dictionary_emails = Arc::new(ReadonlyRepository::<Email>::new(
        Arc::clone(&connection),
        dictionary_options("Email", "emails", &email_columns, &snake)?,
    )?);
    let mut dictionary_contacts = ReadonlyRepository::<Contact>::new(
        Arc::clone(&connection),
        dictionary_options("Contact", "contacts", &contact_columns, &snake)?,
    )?;
    dictionary_contacts.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&dictionary_addresses),
        None,
    );
    dictionary_contacts.add_enriched(
        Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&dictionary_emails),
        None,
    );

    let reads = json!({
        "default": profile(read_iters, rows, || async {
            let loaded: Vec<Contact> = IReadonlyRepository::read(&default_repo, None, None)
                .try_collect()
                .await?;
            check(&loaded, rows, "default")?;
            Ok(())
        })
        .await?,
        "dictionary": profile(read_iters, rows, || async {
            let loaded: Vec<Contact> = IReadonlyRepository::read(&dictionary_contacts, None, None)
                .try_collect()
                .await?;
            check(&loaded, rows, "dictionary")?;
            Ok(())
        })
        .await?,
    });

    println!(
        "{}",
        json!({
            "sample": "mapping_comparison",
            "language": "rust",
            "provider": provider,
            "rows": rows,
            "chunk": chunk.min(rows as usize),
            "createMs": create_ms,
            "createPerSec": if create_ms == 0.0 { 0.0 } else { rows as f64 / (create_ms / 1000.0) },
            "counts": counts,
            "reads": reads,
        })
    );
    connection.close().await.ok();
    Ok(())
}

fn repo(
    connection: Arc<dyn Connection>,
    batched: bool,
    entity: &str,
    table: &str,
    properties: &[(&str, &str)],
    columns: &[&str],
    ids: &[&str],
    create: &str,
    update: &str,
    delete: &str,
) -> Result<BulkCRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = ids.iter().map(|id| (*id).to_string()).collect();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([(
        entity.to_string(),
        table.to_string(),
    )])));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new(
        properties
            .iter()
            .map(|(from, to)| ((*from).to_string(), (*to).to_string())),
    )));
    options.read_properties = columns.iter().map(|column| (*column).to_string()).collect();
    options.create_properties = Some(columns.iter().map(|column| (*column).to_string()).collect());
    options.update_properties = options.create_properties.clone();
    options.row_converter = Some(RowTypeConverter::new(
        columns.iter().map(|column| (*column).to_string()).collect(),
    ));
    options.schema = Schema::new(
        columns
            .iter()
            .map(|column| {
                (
                    (*column).to_string(),
                    SchemaColumn {
                        type_name: column_type(column).into(),
                        nullable: *column == "latitude" || *column == "longitude",
                        has_default: matches!(
                            *column,
                            "uuid" | "created" | "updated" | "version" | "display_name" | "line_2"
                                | "line_3" | "neighborhood" | "district" | "state_province"
                                | "postal_code" | "formatted_address" | "is_primary"
                                | "additional_name" | "pronunciation" | "family_name" | "nick_name"
                                | "job_title" | "organization"
                        ),
                        is_id: *column == "id",
                    },
                )
            })
            .collect(),
    );
    if batched {
        BulkCRUDRepository::with_strategy(
            connection,
            options,
            Arc::new(BatchCommandIBulkWriteStrategy::new()),
        )
    } else {
        BulkCRUDRepository::with_strategy(
            connection,
            options,
            Arc::new(StoredProcedureBulkWriteStrategy::new(create, update, delete)),
        )
    }
}

fn dictionary_options(
    entity: &str,
    table: &str,
    columns: &[&str],
    snake: &ChangeCasingMapper,
) -> Result<ReadonlyRepositoryMappingOptions> {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = entity.into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([(
        entity.to_string(),
        table.to_string(),
    )])));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new(
        columns
            .iter()
            .map(|name| ((*name).to_string(), snake.map((*name).to_string()))),
    )));
    options.read_properties = columns.iter().map(|column| (*column).to_string()).collect();
    options.schema = Schema::new(
        columns
            .iter()
            .map(|column| {
                (
                    (*column).to_string(),
                    SchemaColumn {
                        type_name: column_type(column).into(),
                        nullable: *column == "latitude" || *column == "longitude",
                        has_default: false,
                        is_id: *column == "id",
                    },
                )
            })
            .collect(),
    );
    Ok(options)
}

fn column_type(column: &str) -> &'static str {
    match column {
        "id" | "contact_id" | "gender_type_id" | "email_type_id" => "integer",
        "uuid" => "uuid",
        "created" | "updated" => "datetime",
        "version" => "binary",
        "is_starred" | "is_favorite" | "is_deleted" | "is_primary" => "boolean",
        "latitude" | "longitude" => "decimal",
        _ => "string",
    }
}

fn seed_contacts(total: i64) -> Vec<Row> {
    let mut state = 1u32;
    let mut next = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        state as f64 / 0x1_0000_0000u64 as f64
    };
    let mut related = |max: i64| 2 + (next() * (max - 1) as f64) as i64;
    let created = DateTime::parse_from_rfc3339("2024-01-01T00:00:00.000Z")
        .expect("created")
        .with_timezone(&Utc);
    (1..=total)
        .map(|id| {
            let address_count = related(10);
            let email_count = related(6);
            let addresses = (0..address_count)
                .map(|j| {
                    stamp_row(
                        0,
                        format!("a{id}-{j}"),
                        created,
                        &[
                            ("line_1", Value::from(format!("{id} Line {}", j + 1))),
                            ("city", Value::from(format!("City{}", (id + j) % 50))),
                            ("country_code", Value::from("US")),
                            ("is_primary", Value::Bool(j == 0)),
                        ],
                    )
                })
                .collect::<Vec<_>>();
            let emails = (0..email_count)
                .map(|j| {
                    stamp_row(
                        0,
                        format!("e{id}-{j}"),
                        created,
                        &[
                            ("address", Value::from(format!("c{id}.e{}@example.com", j + 1))),
                            ("display_name", Value::from(format!("Email {}", j + 1))),
                            ("email_type_id", Value::Int(j + 1)),
                            ("is_primary", Value::Bool(j == 0)),
                        ],
                    )
                })
                .collect::<Vec<_>>();
            let mut contact = stamp_row(
                id,
                format!("00000000-0000-0000-0000-{id:012x}"),
                created,
                &[
                    ("name", Value::from(format!("Contact {id}"))),
                    ("given_name", Value::from(format!("Given{id}"))),
                    ("family_name", Value::from(format!("Family{id}"))),
                    ("gender_type_id", Value::Int(1 + (id % 3))),
                    ("is_starred", Value::Bool(false)),
                    ("is_favorite", Value::Bool(false)),
                    ("is_deleted", Value::Bool(false)),
                ],
            );
            contact.insert("addresses".into(), encode_rows(&addresses));
            contact.insert("emails".into(), encode_rows(&emails));
            contact
        })
        .collect()
}

fn stamp_row(id: i64, uuid: String, created: DateTime<Utc>, extra: &[(&str, Value)]) -> Row {
    let mut row = Row::new();
    row.insert("id".into(), Value::Int(id));
    row.insert("uuid".into(), Value::from(uuid));
    row.insert("created".into(), Value::DateTime(created));
    row.insert("updated".into(), Value::DateTime(created));
    row.insert("version".into(), Value::Binary(vec![0]));
    for (name, value) in extra {
        row.insert((*name).into(), value.clone());
    }
    row
}

fn related_len(row: &Row, field: &str) -> usize {
    match row.get(field) {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').count(),
        Some(Value::Array(items)) => items.len(),
        _ => 0,
    }
}

async fn each<F, Fut>(items: &mut [Row], chunk: usize, mut write: F) -> Result<()>
where
    F: FnMut(&mut [Row]) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let mut at = 0;
    while at < items.len() {
        let end = (at + chunk).min(items.len());
        write(&mut items[at..end]).await?;
        at = end;
    }
    Ok(())
}

async fn count(connection: &Arc<dyn Connection>, table: &str) -> Result<i64> {
    let reader = connection
        .create_command()
        .execute_reader_native(&Native::new(format!("SELECT COUNT(*) AS n FROM {table}")), None)
        .await?;
    reader.next().await?;
    match reader.current().and_then(|row| row.get("n").cloned()) {
        Some(Value::Int(n)) => Ok(n),
        Some(Value::Text(text)) => Ok(text.parse().unwrap_or(0)),
        _ => Ok(0),
    }
}

async fn profile<F, Fut>(iters: usize, rows: i64, mut work: F) -> Result<serde_json::Value>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    work().await?;
    let mut times = Vec::with_capacity(iters);
    for _ in 0..iters {
        let started = Instant::now();
        work().await?;
        times.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    times.sort_by(|left, right| left.partial_cmp(right).unwrap());
    let ms = if times.len() % 2 == 1 {
        times[times.len() / 2]
    } else {
        (times[times.len() / 2 - 1] + times[times.len() / 2]) / 2.0
    };
    Ok(json!({ "ms": ms, "perSec": rows as f64 / (ms / 1000.0) }))
}

fn check(loaded: &[Contact], rows: i64, label: &str) -> Result<()> {
    if loaded.len() as i64 != rows {
        return Err(datasources::DatasourceError::message(format!(
            "{label} count {}",
            loaded.len()
        )));
    }
    if loaded.iter().any(|row| {
        row.addresses.len() < 2
            || row.addresses.len() > 10
            || row.emails.len() < 2
            || row.emails.len() > 6
    }) {
        return Err(datasources::DatasourceError::message(format!(
            "{label} related"
        )));
    }
    Ok(())
}

fn env(name: &str, fallback: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| fallback.into())
}
