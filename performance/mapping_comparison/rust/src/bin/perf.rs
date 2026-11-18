use chrono::{DateTime, Datelike, Utc};
use datasources::{
    encode_rows, create, BatchCommandIBulkWriteStrategy, BulkCRUDRepository,
    CRUDRepositoryOptions, ChangeCasingMapper, Connection, Casing, Datasource, DatasourceUrlParser,
    DictionaryMapper, Enrichment, EnrichmentPropertyType, IReadonlyRepository, Native,
    ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result, Row, Schema, SchemaColumn,
    StoredProcedureBulkWriteStrategy, Value,
};
use futures_util::TryStreamExt;
use mapping_comparison::datasources_providers;
use mapping_comparison::generated::repositories::contactsRepository::ContactsRepository;
use mapping_comparison::generated::{Address, Contact, Email, Repositories};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let process_started = Instant::now();
    datasources_providers::register();
    let provider = env("PROVIDER", "sqlite");
    let rows = env("PERF_ROWS", "25000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let startup_ms = process_started.elapsed().as_secs_f64() * 1000.0;
    let (default_read, dictionary_read, property_mapped_read) = tokio::try_join!(
        performance_test_default_read(),
        performance_test_dictionary_read(),
        performance_test_property_mapped_read(),
    )?;
    println!(
        "{}",
        json!({
            "sample": "mapping_comparison",
            "language": "rust",
            "provider": provider,
            "rows": rows,
            "chunk": chunk.min(rows as usize),
            "startupMs": startup_ms,
            "tests": {
                "defaultRead": default_read,
                "dictionaryRead": dictionary_read,
                "propertyMappedRead": property_mapped_read,
            },
        })
    );
    Ok(())
}

const CONTACT_COLUMNS: [&str; 17] = [
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
const ADDRESS_COLUMNS: [&str; 20] = [
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
const EMAIL_COLUMNS: [&str; 10] = [
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

async fn performance_test_default_read() -> Result<serde_json::Value> {
    let provider = env("PROVIDER", "sqlite");
    let url = isolate("default_read").await?;
    let ds = datasource(&provider, &url)?;
    let connection = ds.create_connection().await?;
    connection.open().await?;
    let batched = provider == "sqlite";
    let rows = env("PERF_ROWS", "25000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let read_iters = env("PERF_READ_ITERS", "3").parse::<usize>().expect("iters");
    let contacts_bulk = bulk_contacts(Arc::clone(&connection), batched)?;
    let setup = {
        let contacts_bulk = Arc::clone(&contacts_bulk);
        let connection = Arc::clone(&connection);
        async move {
            let mut seeded = seed_contacts(rows);
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>(started.elapsed().as_secs_f64() * 1000.0)
        }
    };
    let performance_default_read = {
        let ds = Arc::clone(&ds);
        async move {
            let repo = ds.contacts();
            check(&load_contacts(repo.as_ref()).await?, rows, "default")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(repo.as_ref()).await?;
                check(&loaded, rows, "default")?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            times.sort_by(|left, right| left.partial_cmp(right).unwrap());
            let ms = median(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.emails.len()).sum::<usize>(),
                }
            }))
        }
    };
    let teardown = {
        let connection = Arc::clone(&connection);
        async move {
            let _ = connection.close().await;
            drop_isolated("default_read").await
        }
    };
    let seed_ms = setup.await?;
    let read = performance_default_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "readMs": read["ms"],
        "readPerSec": read["perSec"],
        "counts": read["counts"],
    }))
}

async fn performance_test_dictionary_read() -> Result<serde_json::Value> {
    let provider = env("PROVIDER", "sqlite");
    let url = isolate("dictionary_read").await?;
    let ds = datasource(&provider, &url)?;
    let connection = ds.create_connection().await?;
    let _ds = ds;
    connection.open().await?;
    let batched = provider == "sqlite";
    let rows = env("PERF_ROWS", "25000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let read_iters = env("PERF_READ_ITERS", "3").parse::<usize>().expect("iters");
    let contacts_bulk = bulk_contacts(Arc::clone(&connection), batched)?;
    let snake = ChangeCasingMapper::new(Casing::Snake, Casing::Snake);
    let addresses = Arc::new(ReadonlyRepository::<Address>::new(
        Arc::clone(&connection),
        dictionary_options("Address", "addresses", &ADDRESS_COLUMNS, &snake)?,
    )?);
    let emails = Arc::new(ReadonlyRepository::<Email>::new(
        Arc::clone(&connection),
        dictionary_options("Email", "emails", &EMAIL_COLUMNS, &snake)?,
    )?);
    let mut contacts = ReadonlyRepository::<Contact>::new(
        Arc::clone(&connection),
        dictionary_options("Contact", "contacts", &CONTACT_COLUMNS, &snake)?,
    )?;
    contacts.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&addresses),
        None,
    );
    contacts.add_enriched(
        Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&emails),
        None,
    );
    let contacts = Arc::new(contacts);
    let setup = {
        let contacts_bulk = Arc::clone(&contacts_bulk);
        let connection = Arc::clone(&connection);
        async move {
            let mut seeded = seed_contacts(rows);
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>(started.elapsed().as_secs_f64() * 1000.0)
        }
    };
    let performance_dictionary_read = {
        let contacts = Arc::clone(&contacts);
        async move {
            check(&load_contacts(contacts.as_ref()).await?, rows, "dictionary")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(contacts.as_ref()).await?;
                check(&loaded, rows, "dictionary")?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            times.sort_by(|left, right| left.partial_cmp(right).unwrap());
            let ms = median(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.emails.len()).sum::<usize>(),
                }
            }))
        }
    };
    let teardown = {
        let connection = Arc::clone(&connection);
        async move {
            let _ = connection.close().await;
            drop_isolated("dictionary_read").await
        }
    };
    let seed_ms = setup.await?;
    let read = performance_dictionary_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "readMs": read["ms"],
        "readPerSec": read["perSec"],
        "counts": read["counts"],
    }))
}

async fn performance_test_property_mapped_read() -> Result<serde_json::Value> {
    let provider = env("PROVIDER", "sqlite");
    let url = isolate("property_mapped_read").await?;
    let ds = datasource(&provider, &url)?;
    let connection = ds.create_connection().await?;
    let _ds = ds;
    connection.open().await?;
    let batched = provider == "sqlite";
    let rows = env("PERF_ROWS", "25000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let read_iters = env("PERF_READ_ITERS", "3").parse::<usize>().expect("iters");
    let contacts_bulk = bulk_contacts(Arc::clone(&connection), batched)?;
    let setup = {
        let contacts_bulk = Arc::clone(&contacts_bulk);
        let connection = Arc::clone(&connection);
        async move {
            let mut seeded = seed_contacts(rows);
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>(started.elapsed().as_secs_f64() * 1000.0)
        }
    };
    let performance_property_mapped_read = {
        let connection = Arc::clone(&connection);
        async move {
            let repo = ContactsRepository::create_readonly(connection)?;
            check(&load_contacts(&repo).await?, rows, "propertyMapped")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(&repo).await?;
                check(&loaded, rows, "propertyMapped")?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            times.sort_by(|left, right| left.partial_cmp(right).unwrap());
            let ms = median(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.emails.len()).sum::<usize>(),
                }
            }))
        }
    };
    let teardown = {
        let connection = Arc::clone(&connection);
        async move {
            let _ = connection.close().await;
            drop_isolated("property_mapped_read").await
        }
    };
    let seed_ms = setup.await?;
    let read = performance_property_mapped_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "readMs": read["ms"],
        "readPerSec": read["perSec"],
        "counts": read["counts"],
    }))
}

fn datasource(provider: &str, url: &str) -> Result<Arc<dyn Datasource>> {
    let mut options = HashMap::new();
    options.insert("PROVIDER".into(), provider.to_string());
    options.insert("DATASOURCE_URL".into(), url.to_string());
    create(Some(&options))
}

async fn isolate(name: &str) -> Result<String> {
    let provider = env("PROVIDER", "sqlite");
    let source_url = env("DATASOURCE_URL", "sqlite:///./app.sqlite");
    let catalog = format!("mapping_{name}");
    let url = with_datasource(&source_url, &catalog);
    if provider == "sqlite" {
        if let Some(file) = DatasourceUrlParser::parse(&url)
            .and_then(|parsed| parsed.datasource().map(|path| path.to_string()))
        {
            let _ = std::fs::remove_file(&file);
            let _ = std::fs::remove_file(format!("{file}-wal"));
            let _ = std::fs::remove_file(format!("{file}-shm"));
        }
        migrate_up(&provider, &url)?;
        return Ok(url);
    }
    let admin_url = if provider == "postgres" {
        with_datasource(&source_url, "postgres")
    } else {
        source_url.clone()
    };
    let ds = datasource(&provider, &admin_url)?;
    let connection = ds.create_connection().await?;
    connection.open().await?;
    let result = async {
        if provider == "postgres" {
            exec(&connection, &format!("DROP DATABASE IF EXISTS {catalog} WITH (FORCE)")).await?;
            exec(&connection, &format!("CREATE DATABASE {catalog}")).await
        } else {
            exec(&connection, &format!("DROP DATABASE IF EXISTS `{catalog}`")).await?;
            exec(&connection, &format!("CREATE DATABASE `{catalog}`")).await
        }
    }
    .await;
    let _ = connection.close().await;
    result?;
    migrate_up(&provider, &url)?;
    Ok(url)
}

fn migrate_up(provider: &str, url: &str) -> Result<()> {
    npx(&["migrate-setup", "--provider", provider, "--connection", url])?;
    npx(&[
        "migrate-up",
        "--provider",
        provider,
        "--connection",
        url,
        "--migrations-root",
        "sql",
    ])
}

fn npx(args: &[&str]) -> Result<()> {
    let output = std::process::Command::new("npx")
        .args(args)
        .output()
        .map_err(|error| datasources::DatasourceError::message(error.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(datasources::DatasourceError::message(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

async fn drop_isolated(name: &str) -> Result<()> {
    let provider = env("PROVIDER", "sqlite");
    let source_url = env("DATASOURCE_URL", "sqlite:///./app.sqlite");
    let catalog = format!("mapping_{name}");
    if provider == "sqlite" {
        if let Some(file) = DatasourceUrlParser::parse(&with_datasource(&source_url, &catalog))
            .and_then(|parsed| parsed.datasource().map(|path| path.to_string()))
        {
            let _ = std::fs::remove_file(&file);
            let _ = std::fs::remove_file(format!("{file}-wal"));
            let _ = std::fs::remove_file(format!("{file}-shm"));
        }
        return Ok(());
    }
    let admin_url = if provider == "postgres" {
        with_datasource(&source_url, "postgres")
    } else {
        source_url
    };
    let ds = datasource(&provider, &admin_url)?;
    let connection = ds.create_connection().await?;
    connection.open().await?;
    let sql = if provider == "postgres" {
        format!("DROP DATABASE IF EXISTS {catalog} WITH (FORCE)")
    } else {
        format!("DROP DATABASE IF EXISTS `{catalog}`")
    };
    let result = exec(&connection, &sql).await;
    let _ = connection.close().await;
    result
}

fn with_datasource(url: &str, name: &str) -> String {
    if let Some(parsed) = DatasourceUrlParser::parse(url) {
        if parsed.provider == "sqlite" {
            let file = parsed.datasource().unwrap_or("app.sqlite");
            let dir = std::path::Path::new(file)
                .parent()
                .and_then(|path| path.to_str())
                .filter(|path| !path.is_empty())
                .unwrap_or(".");
            return format!("sqlite:///{dir}/{name}.sqlite");
        }
    }
    match url.rfind('/') {
        Some(slash) => match url[slash + 1..].find('?') {
            Some(query) => format!("{}{name}{}", &url[..=slash], &url[slash + 1 + query..]),
            None => format!("{}{name}", &url[..=slash]),
        },
        None => url.to_string(),
    }
}

async fn exec(connection: &Arc<dyn Connection>, sql: &str) -> Result<()> {
    connection
        .create_command()
        .execute_native(&Native::new(sql), None)
        .await?;
    Ok(())
}

fn bulk_contacts(connection: Arc<dyn Connection>, batched: bool) -> Result<Arc<BulkCRUDRepository>> {
    let addresses = Arc::new(repo(
        Arc::clone(&connection),
        batched,
        "Address",
        "addresses",
        &ADDRESS_COLUMNS,
        "bulk_create_addresses",
        "bulk_update_addresses",
        "bulk_delete_addresses",
    )?);
    let emails = Arc::new(repo(
        Arc::clone(&connection),
        batched,
        "Email",
        "emails",
        &EMAIL_COLUMNS,
        "bulk_create_emails",
        "bulk_update_emails",
        "bulk_delete_emails",
    )?);
    let mut contacts = repo(
        connection,
        batched,
        "Contact",
        "contacts",
        &CONTACT_COLUMNS,
        "bulk_create_contacts",
        "bulk_update_contacts",
        "bulk_delete_contacts",
    )?;
    contacts.add_enriched(
        Enrichment::new("addresses", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&addresses),
        None,
    );
    contacts.add_enriched(
        Enrichment::new("emails", "id", "contact_id", None, EnrichmentPropertyType::List),
        Arc::clone(&emails),
        None,
    );
    Ok(Arc::new(contacts))
}

fn repo(
    connection: Arc<dyn Connection>,
    batched: bool,
    entity: &str,
    table: &str,
    columns: &[&str],
    create: &str,
    update: &str,
    delete: &str,
) -> Result<BulkCRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["id".into()];
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([(
        entity.to_string(),
        table.to_string(),
    )])));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new(
        std::iter::empty::<(String, String)>(),
    )));
    options.read_properties = columns.iter().map(|column| (*column).to_string()).collect();
    options.create_properties = Some(columns.iter().map(|column| (*column).to_string()).collect());
    options.update_properties = options.create_properties.clone();
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
                            "uuid"
                                | "created"
                                | "updated"
                                | "version"
                                | "display_name"
                                | "line_2"
                                | "line_3"
                                | "neighborhood"
                                | "district"
                                | "state_province"
                                | "postal_code"
                                | "formatted_address"
                                | "is_primary"
                                | "additional_name"
                                | "pronunciation"
                                | "family_name"
                                | "nick_name"
                                | "job_title"
                                | "organization"
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
    let created = DateTime::parse_from_rfc3339("2024-01-01T00:00:00.000Z")
        .expect("created")
        .with_timezone(&Utc);
    (1..=total)
        .map(|id| {
            let addresses = (0..2)
                .map(|j| {
                    stamp_row(
                        0,
                        datasources::uuid::Uuid::new_v4(),
                        created,
                        &[
                            ("display_name", Value::from("")),
                            ("line_1", Value::from(format!("{id} Line {}", j + 1))),
                            ("line_2", Value::from("")),
                            ("line_3", Value::from("")),
                            ("neighborhood", Value::from("")),
                            ("district", Value::from("")),
                            ("city", Value::from(format!("City{}", (id + j) % 50))),
                            ("state_province", Value::from("")),
                            ("postal_code", Value::from("")),
                            ("country_code", Value::from("US")),
                            ("formatted_address", Value::from("")),
                            ("is_primary", Value::Bool(j == 0)),
                        ],
                    )
                })
                .collect::<Vec<_>>();
            let emails = (0..2)
                .map(|j| {
                    stamp_row(
                        0,
                        datasources::uuid::Uuid::new_v4(),
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
                0,
                datasources::uuid::Uuid::new_v4(),
                created,
                &[
                    ("name", Value::from(format!("Contact {id}"))),
                    ("given_name", Value::from(format!("Given{id}"))),
                    ("additional_name", Value::from("")),
                    ("pronunciation", Value::from("")),
                    ("family_name", Value::from(format!("Family{id}"))),
                    ("nick_name", Value::from("")),
                    ("job_title", Value::from("")),
                    ("organization", Value::from("")),
                    ("gender_type_id", Value::Int(1 + (id % 3))),
                    ("is_starred", Value::Bool(id % 2 == 0)),
                    ("is_favorite", Value::Bool(id % 3 == 0)),
                    ("is_deleted", Value::Bool(id % 5 == 0)),
                ],
            );
            contact.insert("addresses".into(), encode_rows(&addresses));
            contact.insert("emails".into(), encode_rows(&emails));
            contact
        })
        .collect()
}

fn stamp_row(id: i64, uuid: datasources::uuid::Uuid, created: DateTime<Utc>, extra: &[(&str, Value)]) -> Row {
    let mut row = Row::new();
    row.insert("id".into(), Value::Int(id));
    row.insert("uuid".into(), Value::Uuid(uuid));
    row.insert("created".into(), Value::DateTime(created));
    row.insert("updated".into(), Value::DateTime(created));
    row.insert("version".into(), Value::Binary(vec![0]));
    for (name, value) in extra {
        row.insert((*name).into(), value.clone());
    }
    row
}

async fn count_contacts(connection: &Arc<dyn Connection>, rows: i64) -> Result<()> {
    let n = count(connection, "contacts").await?;
    if n != rows {
        return Err(datasources::DatasourceError::message(format!(
            "create count contacts={n}"
        )));
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

async fn load_contacts<T: IReadonlyRepository<Contact> + ?Sized>(repo: &T) -> Result<Vec<Contact>> {
    IReadonlyRepository::read(repo, None, None).try_collect().await
}

fn mapped_date(value: DateTime<Utc>) -> bool {
    value.year() >= 2020
}

fn check(loaded: &[Contact], rows: i64, label: &str) -> Result<()> {
    if loaded.len() as i64 != rows {
        return Err(datasources::DatasourceError::message(format!(
            "{label} count {}",
            loaded.len()
        )));
    }
    for row in loaded {
        if row.addresses.len() != 2 || row.emails.len() != 2 {
            return Err(datasources::DatasourceError::message(format!(
                "{label} related {}",
                row.given_name
            )));
        }
        if row.uuid.is_nil()
            || row.addresses.iter().any(|item| item.uuid.is_nil())
            || row.emails.iter().any(|item| item.uuid.is_nil())
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} uuid {}",
                row.given_name
            )));
        }
        if !mapped_date(row.created)
            || !mapped_date(row.updated)
            || row
                .addresses
                .iter()
                .any(|item| !mapped_date(item.created) || !mapped_date(item.updated))
            || row
                .emails
                .iter()
                .any(|item| !mapped_date(item.created) || !mapped_date(item.updated))
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} datetime {}",
                row.given_name
            )));
        }
        let n = row
            .given_name
            .strip_prefix("Given")
            .and_then(|rest| rest.parse::<i64>().ok())
            .ok_or_else(|| {
                datasources::DatasourceError::message(format!("{label} givenName {}", row.given_name))
            })?;
        if row.is_starred != (n % 2 == 0)
            || row.is_favorite != (n % 3 == 0)
            || row.is_deleted != (n % 5 == 0)
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} boolean {}",
                row.given_name
            )));
        }
        if !row.addresses[0].is_primary
            || row.addresses[1].is_primary
            || !row.emails[0].is_primary
            || row.emails[1].is_primary
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} isPrimary {}",
                row.given_name
            )));
        }
    }
    Ok(())
}

fn median(times: &[f64]) -> f64 {
    if times.len() % 2 == 1 {
        times[times.len() / 2]
    } else {
        (times[times.len() / 2 - 1] + times[times.len() / 2]) / 2.0
    }
}

fn env(name: &str, fallback: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| fallback.into())
}
