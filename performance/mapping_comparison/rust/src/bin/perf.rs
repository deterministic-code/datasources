use chrono::{DateTime, Datelike, Utc};
use datasources::{
    encode_rows, create, BatchCommandIBulkWriteStrategy, BulkCRUDRepository, BulkWriteStrategy,
    ChangeCasingMapper, Connection, Casing, Datasource, DatasourceUrlParser,
    Association, DictionaryMapper, Enrichment, EnrichmentPropertyType, GraphReadonlyEnrichmentStrategy,
    IReadonlyRepository, Native,
    ReadonlyRepository, ReadonlyRepositoryMappingOptions, Result, Row, Schema, SchemaColumn,
    StoredProcedureBulkWriteStrategy, Value,
};
use futures_util::TryStreamExt;
use mapping_comparison::datasources_providers;
use mapping_comparison::generated::repositories::addressesRepository::AddressesRepository;
use mapping_comparison::generated::repositories::contactAttributesRepository::ContactAttributesRepository;
use mapping_comparison::generated::repositories::contactLabelsRepository::ContactLabelsRepository;
use mapping_comparison::generated::repositories::contactsRepository::ContactsRepository;
use mapping_comparison::generated::repositories::emailsRepository::EmailsRepository;
use mapping_comparison::generated::repositories::labelsRepository::LabelsRepository;
use mapping_comparison::generated::repositories::phonesRepository::PhonesRepository;
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
    let default_read = performance_test_default_read().await?;
    let dictionary_read = performance_test_dictionary_read().await?;
    let property_mapped_read = performance_test_property_mapped_read().await?;
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

const CONTACT_PROPERTIES: [&str; 17] = [
    "Id",
    "Uuid",
    "Created",
    "Updated",
    "Version",
    "Name",
    "GivenName",
    "AdditionalName",
    "Pronunciation",
    "FamilyName",
    "NickName",
    "JobTitle",
    "Organization",
    "IsStarred",
    "IsFavorite",
    "IsDeleted",
    "GenderTypeId",
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
            let cpu = cpu_ms();
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            let seed_ms = started.elapsed().as_secs_f64() * 1000.0;
            let seed_cpu_ms = cpu_ms() - cpu;
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>((seed_ms, seed_cpu_ms))
        }
    };
    let performance_default_read = {
        let ds = Arc::clone(&ds);
        async move {
            let repo = ds.Contacts();
            check(&load_contacts(repo.as_ref()).await?, rows, "default")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            let cpu = cpu_ms();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(repo.as_ref()).await?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            let cpu_used = cpu_ms() - cpu;
            check(&loaded, rows, "default")?;
            let ms = average(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "cpuMs": cpu_used,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.Addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.Emails.len()).sum::<usize>(),
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
    let (seed_ms, seed_cpu_ms) = setup.await?;
    let read = performance_default_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "seedCpuMs": seed_cpu_ms,
        "readMs": read["ms"],
        "readCpuMs": read["cpuMs"],
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
    let snake = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
    let address = AddressesRepository::new(Arc::clone(&connection))?;
    let email = EmailsRepository::new(Arc::clone(&connection))?;
    let phone = PhonesRepository::new(Arc::clone(&connection))?;
    let contact_attribute = ContactAttributesRepository::new(Arc::clone(&connection))?;
    let label = LabelsRepository::new(Arc::clone(&connection))?;
    let contact_label = ContactLabelsRepository::new(Arc::clone(&connection))?;
    let mut contacts = ReadonlyRepository::<Contact>::new(
        Arc::clone(&connection),
        dictionary_options("Contact", "contacts", &CONTACT_PROPERTIES, &snake)?,
    )?;
    contacts.add_enriched(
        Enrichment::new("Addresses", "Id", "ContactId", None, EnrichmentPropertyType::List),
        &address,
        None,
    );
    contacts.add_enriched(
        Enrichment::new("Emails", "Id", "ContactId", None, EnrichmentPropertyType::List),
        &email,
        None,
    );
    contacts.add_enriched(
        Enrichment::new("Phones", "Id", "ContactId", None, EnrichmentPropertyType::List),
        &phone,
        None,
    );
    contacts.add_enriched(
        Enrichment::new("Attributes", "Id", "ContactId", Some("Value".into()), EnrichmentPropertyType::Dictionary)
            .with_dictionary_key("Name"),
        &contact_attribute,
        None,
    );
    contacts.add_enriched(
        Enrichment::new("Labels", "Id", "Id", Some("Name".into()), EnrichmentPropertyType::List)
            .linked(Association::new("ContactId", "LabelId")),
        &label,
        Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(&contact_label))),
    );
    let contacts = Arc::new(contacts);
    let setup = {
        let contacts_bulk = Arc::clone(&contacts_bulk);
        let connection = Arc::clone(&connection);
        async move {
            let mut seeded = seed_contacts(rows);
            let cpu = cpu_ms();
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            let seed_ms = started.elapsed().as_secs_f64() * 1000.0;
            let seed_cpu_ms = cpu_ms() - cpu;
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>((seed_ms, seed_cpu_ms))
        }
    };
    let performance_dictionary_read = {
        let contacts = Arc::clone(&contacts);
        async move {
            check(&load_contacts(contacts.as_ref()).await?, rows, "dictionary")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            let cpu = cpu_ms();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(contacts.as_ref()).await?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            let cpu_used = cpu_ms() - cpu;
            check(&loaded, rows, "dictionary")?;
            let ms = average(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "cpuMs": cpu_used,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.Addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.Emails.len()).sum::<usize>(),
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
    let (seed_ms, seed_cpu_ms) = setup.await?;
    let read = performance_dictionary_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "seedCpuMs": seed_cpu_ms,
        "readMs": read["ms"],
        "readCpuMs": read["cpuMs"],
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
            let cpu = cpu_ms();
            let started = Instant::now();
            let mut at = 0;
            while at < seeded.len() {
                let end = (at + chunk).min(seeded.len());
                contacts_bulk.create_bulk(&mut seeded[at..end]).await?;
                at = end;
            }
            let seed_ms = started.elapsed().as_secs_f64() * 1000.0;
            let seed_cpu_ms = cpu_ms() - cpu;
            count_contacts(&connection, rows).await?;
            Ok::<_, datasources::DatasourceError>((seed_ms, seed_cpu_ms))
        }
    };
    let performance_property_mapped_read = {
        let connection = Arc::clone(&connection);
        async move {
            let repo = ContactsRepository::create_readonly(connection)?;
            check(&load_contacts(&repo).await?, rows, "propertyMapped")?;
            let mut times = Vec::with_capacity(read_iters);
            let mut loaded = Vec::new();
            let cpu = cpu_ms();
            for _ in 0..read_iters {
                let started = Instant::now();
                loaded = load_contacts(&repo).await?;
                times.push(started.elapsed().as_secs_f64() * 1000.0);
            }
            let cpu_used = cpu_ms() - cpu;
            check(&loaded, rows, "propertyMapped")?;
            let ms = average(&times);
            Ok::<_, datasources::DatasourceError>(json!({
                "ms": ms,
                "cpuMs": cpu_used,
                "perSec": rows as f64 / (ms / 1000.0),
                "counts": {
                    "contacts": loaded.len(),
                    "addresses": loaded.iter().map(|row| row.Addresses.len()).sum::<usize>(),
                    "emails": loaded.iter().map(|row| row.Emails.len()).sum::<usize>(),
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
    let (seed_ms, seed_cpu_ms) = setup.await?;
    let read = performance_property_mapped_read.await?;
    teardown.await?;
    Ok(json!({
        "seedMs": seed_ms,
        "seedCpuMs": seed_cpu_ms,
        "readMs": read["ms"],
        "readCpuMs": read["cpuMs"],
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

fn bulk_write(batched: bool, create: &str, update: &str, delete: &str) -> Arc<dyn BulkWriteStrategy> {
    if batched {
        Arc::new(BatchCommandIBulkWriteStrategy::new())
    } else {
        Arc::new(StoredProcedureBulkWriteStrategy::new(create, update, delete))
    }
}

fn bulk_contacts(connection: Arc<dyn Connection>, batched: bool) -> Result<Arc<BulkCRUDRepository>> {
    let addresses = Arc::new(BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        AddressesRepository::options()?,
        bulk_write(
            batched,
            "bulk_create_addresses",
            "bulk_update_addresses",
            "bulk_delete_addresses",
        ),
    )?);
    let emails = Arc::new(BulkCRUDRepository::with_strategy(
        Arc::clone(&connection),
        EmailsRepository::options()?,
        bulk_write(
            batched,
            "bulk_create_emails",
            "bulk_update_emails",
            "bulk_delete_emails",
        ),
    )?);
    let mut contacts = BulkCRUDRepository::with_strategy(
        connection,
        ContactsRepository::options()?,
        bulk_write(
            batched,
            "bulk_create_contacts",
            "bulk_update_contacts",
            "bulk_delete_contacts",
        ),
    )?;
    contacts.add_enriched(
        Enrichment::new("Addresses", "Id", "ContactId", None, EnrichmentPropertyType::List),
        Arc::clone(&addresses),
        None,
    );
    contacts.add_enriched(
        Enrichment::new("Emails", "Id", "ContactId", None, EnrichmentPropertyType::List),
        Arc::clone(&emails),
        None,
    );
    Ok(Arc::new(contacts))
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
                        nullable: *column == "Latitude" || *column == "Longitude",
                        has_default: matches!(
                            *column,
                            "Uuid"
                                | "Created"
                                | "Updated"
                                | "Version"
                                | "DisplayName"
                                | "Line2"
                                | "Line3"
                                | "Neighborhood"
                                | "District"
                                | "StateProvince"
                                | "PostalCode"
                                | "FormattedAddress"
                                | "IsPrimary"
                                | "AdditionalName"
                                | "Pronunciation"
                                | "FamilyName"
                                | "NickName"
                                | "JobTitle"
                                | "Organization"
                        ),
                        is_id: *column == "Id",
                    },
                )
            })
            .collect(),
    );
    Ok(options)
}

fn column_type(column: &str) -> &'static str {
    match column {
        "Id" | "ContactId" | "GenderTypeId" | "EmailTypeId" => "integer",
        "Uuid" => "uuid",
        "Created" | "Updated" => "datetime",
        "Version" => "binary",
        "IsStarred" | "IsFavorite" | "IsDeleted" | "IsPrimary" => "boolean",
        "Latitude" | "Longitude" => "decimal",
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
                            ("DisplayName", Value::from("")),
                            ("Line1", Value::from(format!("{id} Line {}", j + 1))),
                            ("Line2", Value::from("")),
                            ("Line3", Value::from("")),
                            ("Neighborhood", Value::from("")),
                            ("District", Value::from("")),
                            ("City", Value::from(format!("City{}", (id + j) % 50))),
                            ("StateProvince", Value::from("")),
                            ("PostalCode", Value::from("")),
                            ("CountryCode", Value::from("US")),
                            ("FormattedAddress", Value::from("")),
                            ("IsPrimary", Value::Bool(j == 0)),
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
                            ("Address", Value::from(format!("c{id}.e{}@example.com", j + 1))),
                            ("DisplayName", Value::from(format!("Email {}", j + 1))),
                            ("EmailTypeId", Value::Int(j + 1)),
                            ("IsPrimary", Value::Bool(j == 0)),
                        ],
                    )
                })
                .collect::<Vec<_>>();
            let mut contact = stamp_row(
                0,
                datasources::uuid::Uuid::new_v4(),
                created,
                &[
                    ("Name", Value::from(format!("Contact {id}"))),
                    ("GivenName", Value::from(format!("Given{id}"))),
                    ("AdditionalName", Value::from("")),
                    ("Pronunciation", Value::from("")),
                    ("FamilyName", Value::from(format!("Family{id}"))),
                    ("NickName", Value::from("")),
                    ("JobTitle", Value::from("")),
                    ("Organization", Value::from("")),
                    ("GenderTypeId", Value::Int(1 + (id % 3))),
                    ("IsStarred", Value::Bool(id % 2 == 0)),
                    ("IsFavorite", Value::Bool(id % 3 == 0)),
                    ("IsDeleted", Value::Bool(id % 5 == 0)),
                ],
            );
            contact.insert("Addresses".into(), encode_rows(&addresses));
            contact.insert("Emails".into(), encode_rows(&emails));
            contact
        })
        .collect()
}

fn stamp_row(id: i64, uuid: datasources::uuid::Uuid, created: DateTime<Utc>, extra: &[(&str, Value)]) -> Row {
    let mut row = Row::new();
    row.insert("Id".into(), Value::Int(id));
    row.insert("Uuid".into(), Value::Uuid(uuid));
    row.insert("Created".into(), Value::DateTime(created));
    row.insert("Updated".into(), Value::DateTime(created));
    row.insert("Version".into(), Value::Binary(vec![0]));
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
        if row.Addresses.len() != 2 || row.Emails.len() != 2 {
            return Err(datasources::DatasourceError::message(format!(
                "{label} related {}",
                row.GivenName
            )));
        }
        if !row.Phones.is_empty() || !row.Attributes.is_empty() || !row.Labels.is_empty() {
            return Err(datasources::DatasourceError::message(format!(
                "{label} empty fills {}",
                row.GivenName
            )));
        }
        if row.Uuid.is_nil()
            || row.Addresses.iter().any(|item| item.Uuid.is_nil())
            || row.Emails.iter().any(|item| item.Uuid.is_nil())
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} uuid {}",
                row.GivenName
            )));
        }
        if !mapped_date(row.Created)
            || !mapped_date(row.Updated)
            || row
                .Addresses
                .iter()
                .any(|item| !mapped_date(item.Created) || !mapped_date(item.Updated))
            || row
                .Emails
                .iter()
                .any(|item| !mapped_date(item.Created) || !mapped_date(item.Updated))
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} datetime {}",
                row.GivenName
            )));
        }
        let n = row
            .GivenName
            .strip_prefix("Given")
            .and_then(|rest| rest.parse::<i64>().ok())
            .ok_or_else(|| {
                datasources::DatasourceError::message(format!("{label} givenName {}", row.GivenName))
            })?;
        if row.IsStarred != (n % 2 == 0)
            || row.IsFavorite != (n % 3 == 0)
            || row.IsDeleted != (n % 5 == 0)
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} boolean {}",
                row.GivenName
            )));
        }
        if !row.Addresses[0].IsPrimary
            || row.Addresses[1].IsPrimary
            || !row.Emails[0].IsPrimary
            || row.Emails[1].IsPrimary
        {
            return Err(datasources::DatasourceError::message(format!(
                "{label} isPrimary {}",
                row.GivenName
            )));
        }
    }
    Ok(())
}

fn cpu_ms() -> f64 {
    let mut time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut time) } != 0 {
        return 0.0;
    }
    time.tv_sec as f64 * 1000.0 + time.tv_nsec as f64 / 1_000_000.0
}

fn average(times: &[f64]) -> f64 {
    times.iter().sum::<f64>() / times.len() as f64
}

fn env(name: &str, fallback: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| fallback.into())
}
