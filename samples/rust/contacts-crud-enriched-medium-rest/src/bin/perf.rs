use chrono::{DateTime, Utc};
use datasources::{
    encode_rows, BatchCommandIBulkWriteStrategy, BulkCRUDRepository, CRUDRepository,
    CRUDRepositoryOptions, Connection, DictionaryMapper, Driver, Enrichment, EnrichmentPropertyType,
    FlatToNestedMapper, HashsetOptimizedReadonlyEnrichmentStrategy, IReadonlyRepository, Join,
    MappedValue, Mapper, Native, Result, Row, RowTypeConverter, SelectStatement,
    SelectStatementOptions,     StoredProcedureBulkWriteStrategy, TypeMap, TypeMapField, TypeMapGroup, Value, WhereText,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

#[derive(Deserialize)]
#[allow(dead_code)]
struct SeedStamp {
    id: i64,
    uuid: String,
    created: String,
    updated: String,
    version: Vec<u8>,
}

#[derive(Deserialize)]
struct SeedAddress {
    #[serde(flatten)]
    stamp: SeedStamp,
    #[serde(rename = "addressTypeId")]
    address_type_id: i64,
    line1: String,
    city: String,
}

#[derive(Deserialize)]
struct SeedPhone {
    #[serde(flatten)]
    stamp: SeedStamp,
    #[serde(rename = "phoneTypeId")]
    phone_type_id: i64,
    number: String,
    label: String,
}

#[derive(Deserialize)]
struct SeedContact {
    #[serde(flatten)]
    stamp: SeedStamp,
    #[serde(rename = "contactSourceId")]
    contact_source_id: i64,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: String,
    notes: Option<String>,
    addresses: Vec<SeedAddress>,
    phones: Vec<SeedPhone>,
    settings: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = env("PROVIDER", "sqlite");
    let strategy_name = env("PERF_STRATEGY", "batch");
    let rows = env("PERF_ROWS", "1000").parse::<i64>().expect("rows");
    let chunk = env("PERF_CHUNK", "500").parse::<usize>().expect("chunk");
    let url = env("DATASOURCE_URL", "sqlite:///:memory:");
    let reading = env("PERF_PHASE", "write") == "read";
    let batched = strategy_name == "batch" || strategy_name == "procedure" && provider == "sqlite";
    let shared = shared();
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), url);
    let driver: Arc<dyn Driver> = match provider.as_str() {
        "sqlite" => Arc::new(datasources_sqlite::SqliteDriver),
        "mysql" => Arc::new(datasources_mysql::MysqlDriver),
        "postgres" => Arc::new(datasources_postgres::PostgresDriver),
        other => return Err(datasources::DatasourceError::message(other)),
    };
    let connection = driver.connection(Some(&options));
    connection.open().await?;
    let command = connection.create_command();
    for sql in split_semi(&std::fs::read_to_string(shared.join(format!("schema-{provider}.sql"))).expect("schema"))
    {
        command.execute_native(&Native::new(sql), None).await?;
    }
    if provider == "mysql" || provider == "postgres" {
        for sql in split_dash(&std::fs::read_to_string(shared.join(format!("bulk-{provider}.sql"))).expect("bulk"))
        {
            command.execute_native(&Native::new(sql), None).await?;
        }
    }
    if !reading {
        let stamp = if provider == "mysql" {
            "2024-01-01 00:00:00".into()
        } else {
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00.000Z")
                .expect("stamp")
                .to_rfc3339()
        };
        let blob = match provider.as_str() {
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
    }
    let settings_repo = Arc::new(repo(
        Arc::clone(&connection),
        &strategy_name,
        batched,
        "ContactSettings",
        "contact_settings",
        &[("contactId", "contact_id")],
        &["contactId", "key", "value"],
        &["contactId", "key"],
        "bulk_create_setting",
        "bulk_update_setting",
        "bulk_delete_setting",
    )?);
    let addresses_repo = Arc::new(repo(
        Arc::clone(&connection),
        &strategy_name,
        batched,
        "Address",
        "addresses",
        &[("contactId", "contact_id"), ("addressTypeId", "address_type_id")],
        &["id", "uuid", "created", "updated", "version", "contactId", "addressTypeId", "line1", "city"],
        &["id"],
        "bulk_create_address",
        "bulk_update_address",
        "bulk_delete_address",
    )?);
    let phones_repo = Arc::new(repo(
        Arc::clone(&connection),
        &strategy_name,
        batched,
        "Phone",
        "phones",
        &[("contactId", "contact_id"), ("phoneTypeId", "phone_type_id")],
        &["id", "uuid", "created", "updated", "version", "contactId", "phoneTypeId", "number", "label"],
        &["id"],
        "bulk_create_phone",
        "bulk_update_phone",
        "bulk_delete_phone",
    )?);
    let mut contacts_repo = repo(
        Arc::clone(&connection),
        &strategy_name,
        batched,
        "Contact",
        "contacts",
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
    enrich(&mut contacts_repo, &addresses_repo, &phones_repo, &settings_repo, None);
    let contacts_repo = Arc::new(contacts_repo);
    let mut create_ms = 0.0;
    let mut update_ms = 0.0;
    let mut related = env("PERF_RELATED", "10").parse::<i64>().expect("related");
    let mut total = rows;
    if !reading {
        let mut contacts = load_contacts(rows);
        let create_started = Instant::now();
        if batched {
            for batch in contacts.chunks_mut(chunk) {
                contacts_repo.create_bulk(batch).await?;
            }
        } else {
            contacts_repo.create_bulk(&mut contacts).await?;
        }
        create_ms = create_started.elapsed().as_secs_f64() * 1000.0;
        let n = contacts.len() as i64;
        related = related_count(&contacts[0]);
        let counts = serde_json::json!({
            "contacts": count(connection.as_ref(), "contacts").await?,
            "addresses": count(connection.as_ref(), "addresses").await?,
            "phones": count(connection.as_ref(), "phones").await?,
            "settings": count(connection.as_ref(), "contact_settings").await?,
        });
        if counts["contacts"] != n || counts["addresses"] != n * related || counts["phones"] != n * related || counts["settings"] != n * related
        {
            return Err(datasources::DatasourceError::message(format!("create count {counts}")));
        }
        for contact in &mut contacts {
            contact.insert("notes".into(), Value::from("updated"));
            mutate_children(contact, "addresses", "city", "City");
            mutate_children(contact, "phones", "label", "w");
            mutate_children(contact, "settings", "value", "dark");
        }
        let update_started = Instant::now();
        if batched {
            for batch in contacts.chunks_mut(chunk) {
                contacts_repo.update_bulk(batch, None).await?;
            }
        } else {
            contacts_repo.update_bulk(&contacts, None).await?;
        }
        update_ms = update_started.elapsed().as_secs_f64() * 1000.0;
        total = n;
    } else {
        total = count(connection.as_ref(), "contacts").await?;
        if total == 0 {
            return Err(datasources::DatasourceError::message(
                "read store is empty; run npm run test:perf first",
            ));
        }
    }
    let counts = serde_json::json!({
        "contacts": count(connection.as_ref(), "contacts").await?,
        "addresses": count(connection.as_ref(), "addresses").await?,
        "phones": count(connection.as_ref(), "phones").await?,
        "settings": count(connection.as_ref(), "contact_settings").await?,
    });
    let mut hashset_repo = repo(
        Arc::clone(&connection),
        &strategy_name,
        batched,
        "Contact",
        "contacts",
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
    let hashset = Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy);
    enrich(
        &mut hashset_repo,
        &addresses_repo,
        &phones_repo,
        &settings_repo,
        Some(hashset),
    );
    let hashset_repo = Arc::new(hashset_repo);
    let join_repo = join_repo(Arc::clone(&connection))?;
    let reads = if reading {
        Some(profile_reads(contacts_repo.as_ref(), hashset_repo.as_ref(), &join_repo, total as usize, related as usize).await?)
    } else {
        None
    };
    println!(
        "{}",
        serde_json::json!({
            "sample": "contacts-crud-enriched-medium-rest",
            "language": "rust",
            "provider": provider,
            "strategy": strategy_name,
            "rows": total,
            "chunk": if batched { chunk.min(total as usize) } else { total as usize },
            "createMs": create_ms,
            "updateMs": update_ms,
            "createPerSec": if create_ms == 0.0 { 0.0 } else { total as f64 / (create_ms / 1000.0) },
            "updatePerSec": if update_ms == 0.0 { 0.0 } else { total as f64 / (update_ms / 1000.0) },
            "counts": counts,
            "reads": reads,
        })
    );
    let _ = connection.close().await;
    Ok(())
}

fn enrich(
    contacts: &mut BulkCRUDRepository,
    addresses: &Arc<BulkCRUDRepository>,
    phones: &Arc<BulkCRUDRepository>,
    settings: &Arc<BulkCRUDRepository>,
    strategy: Option<Arc<dyn datasources::ReadonlyEnrichmentStrategy>>,
) {
    contacts
        .add_enriched(
            Enrichment::new("addresses", "id", "contactId", None, EnrichmentPropertyType::List),
            Arc::clone(addresses),
            strategy.clone(),
        )
        .add_enriched(
            Enrichment::new("phones", "id", "contactId", None, EnrichmentPropertyType::List),
            Arc::clone(phones),
            strategy.clone(),
        )
        .add_enriched(
            Enrichment::new(
                "settings",
                "id",
                "contactId",
                Some("value".into()),
                EnrichmentPropertyType::Dictionary,
            ),
            Arc::clone(settings),
            strategy,
        );
}

fn repo(
    connection: Arc<dyn Connection>,
    strategy_name: &str,
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
    options.row_converter = Some(RowTypeConverter::new(
        columns.iter().map(|column| (*column).to_string()).collect(),
    ));
    match strategy_name {
        "single" => BulkCRUDRepository::new(connection, options),
        _ if batched => BulkCRUDRepository::with_strategy(
            connection,
            options,
            Arc::new(BatchCommandIBulkWriteStrategy::new()),
        ),
        "procedure" => BulkCRUDRepository::with_strategy(
            connection,
            options,
            Arc::new(StoredProcedureBulkWriteStrategy::new(create, update, delete)),
        ),
        other => Err(datasources::DatasourceError::message(other)),
    }
}

fn join_repo(connection: Arc<dyn Connection>) -> Result<CRUDRepository> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Contact".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("Contact".into(), "contacts".into()),
        ("Address".into(), "addresses".into()),
        ("Phone".into(), "phones".into()),
        ("ContactSettings".into(), "contact_settings".into()),
    ])));
    options.property_mappings = Some(Arc::new(DictionaryMapper::new([
        ("firstName".to_string(), "first_name".to_string()),
        ("contactId".to_string(), "contact_id".to_string()),
    ])));
    CRUDRepository::new(connection, options)
}

fn load_contacts(rows: i64) -> Vec<Row> {
    if let Ok(path) = std::env::var("PERF_DATA") {
        let seed: Vec<SeedContact> = serde_json::from_str(&std::fs::read_to_string(path).expect("data")).expect("json");
        return seed.into_iter().map(contact_from_seed).collect();
    }
    let each = env("PERF_RELATED", "10").parse::<i64>().expect("related");
    (1..=rows)
        .map(|id| {
            let created = DateTime::parse_from_rfc3339("2024-01-01T00:00:00.000Z")
                .expect("created")
                .with_timezone(&Utc);
            let addresses = (0..each)
                .map(|n| {
                    let child = (id - 1) * each + n + 1;
                    stamp_row(child, format!("a{child}"), created, &[
                        ("addressTypeId", Value::Int(1)),
                        ("line1", Value::from("1 Main")),
                        ("city", Value::from("Town")),
                    ])
                })
                .collect::<Vec<_>>();
            let phones = (0..each)
                .map(|n| {
                    let child = (id - 1) * each + n + 1;
                    stamp_row(child, format!("p{child}"), created, &[
                        ("phoneTypeId", Value::Int(1)),
                        ("number", Value::from("5550000")),
                        ("label", Value::from("m")),
                    ])
                })
                .collect::<Vec<_>>();
            let settings = (0..each)
                .map(|n| {
                    let mut row = Row::new();
                    row.insert("key".into(), Value::from(format!("k{n}")));
                    row.insert("value".into(), Value::from("light"));
                    row
                })
                .collect::<Vec<_>>();
            let mut contact = stamp_row(id, format!("00000000-0000-0000-0000-{id:012x}"), created, &[
                ("contactSourceId", Value::Int(1)),
                ("firstName", Value::from(format!("First{id}"))),
                ("lastName", Value::from(format!("Last{id}"))),
                ("email", Value::from(format!("c{id}@ex.test"))),
                ("notes", Value::from("")),
            ]);
            contact.insert("addresses".into(), encode_rows(&addresses));
            contact.insert("phones".into(), encode_rows(&phones));
            contact.insert("settings".into(), encode_rows(&settings));
            contact
        })
        .collect()
}

fn contact_from_seed(seed: SeedContact) -> Row {
    let created = parse_date(&seed.stamp.created);
    let addresses = seed
        .addresses
        .into_iter()
        .map(|address| {
            stamp_row(address.stamp.id, address.stamp.uuid, parse_date(&address.stamp.created), &[
                ("addressTypeId", Value::Int(address.address_type_id)),
                ("line1", Value::from(address.line1)),
                ("city", Value::from(address.city)),
            ])
        })
        .collect::<Vec<_>>();
    let phones = seed
        .phones
        .into_iter()
        .map(|phone| {
            stamp_row(phone.stamp.id, phone.stamp.uuid, parse_date(&phone.stamp.created), &[
                ("phoneTypeId", Value::Int(phone.phone_type_id)),
                ("number", Value::from(phone.number)),
                ("label", Value::from(phone.label)),
            ])
        })
        .collect::<Vec<_>>();
    let settings = seed
        .settings
        .into_iter()
        .map(|(key, value)| {
            let mut row = Row::new();
            row.insert("key".into(), Value::from(key));
            row.insert("value".into(), Value::from(value));
            row
        })
        .collect::<Vec<_>>();
    let mut contact = stamp_row(
        seed.stamp.id,
        seed.stamp.uuid,
        created,
        &[
            ("contactSourceId", Value::Int(seed.contact_source_id)),
            ("firstName", Value::from(seed.first_name)),
            ("lastName", Value::from(seed.last_name)),
            ("email", Value::from(seed.email)),
            ("notes", Value::from(seed.notes.unwrap_or_default())),
        ],
    );
    contact.insert("addresses".into(), encode_rows(&addresses));
    contact.insert("phones".into(), encode_rows(&phones));
    contact.insert("settings".into(), encode_rows(&settings));
    contact
}

fn stamp_row(id: i64, uuid: String, created: DateTime<Utc>, extra: &[(&str, Value)]) -> Row {
    let mut row = Row::new();
    row.insert("id".into(), Value::Int(id));
    row.insert("uuid".into(), Value::from(uuid));
    row.insert("created".into(), Value::DateTime(created));
    row.insert("updated".into(), Value::DateTime(created));
    row.insert("version".into(), Value::Binary(vec![1]));
    for (name, value) in extra {
        row.insert((*name).into(), value.clone());
    }
    row
}

fn parse_date(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .unwrap_or_else(|_| DateTime::parse_from_rfc3339("2024-01-01T00:00:00.000Z").expect("date"))
        .with_timezone(&Utc)
}

fn mutate_children(parent: &mut Row, field: &str, key: &str, value: &str) {
    let children = match parent.get(field) {
        Some(Value::Text(text)) if !text.is_empty() => text
            .split('|')
            .map(|row| {
                let mut mapped = decode_row(row);
                mapped.insert(key.into(), Value::from(value));
                mapped
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    parent.insert(field.into(), encode_rows(&children));
}

fn decode_row(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((
                key.to_string(),
                Value::from(
                    value
                        .replace("%7C", "|")
                        .replace("%2C", ",")
                        .replace("%3D", "=")
                        .replace("%25", "%"),
                ),
            ))
        })
        .collect()
}

fn related_count(contact: &Row) -> i64 {
    match contact.get("addresses") {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').count() as i64,
        _ => 0,
    }
}

async fn time<F, Fut>(work: F) -> Result<f64>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let started = Instant::now();
    work().await?;
    Ok(started.elapsed().as_secs_f64() * 1000.0)
}

async fn count(connection: &dyn Connection, table: &str) -> Result<i64> {
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

async fn profile_reads(
    contacts_repo: &BulkCRUDRepository,
    hashset_repo: &BulkCRUDRepository,
    join_repo: &CRUDRepository,
    total: usize,
    related: usize,
) -> Result<serde_json::Value> {
    let items = env("PERF_READ_ITEMS", "100").parse::<usize>().expect("items").min(total);
    let iters = env("PERF_READ_ITERS", "5").parse::<usize>().expect("iters");
    let mut state = 42u32;
    let mapper = FlatToNestedMapper::new(TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("firstName".into(), TypeMapField::column("firstName")),
            (
                "addresses".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "addressId",
                    vec![("id".into(), TypeMapField::column("addressId"))],
                )),
            ),
            (
                "phones".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "phoneId",
                    vec![("id".into(), TypeMapField::column("phoneId"))],
                )),
            ),
            (
                "settings".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "settingKey",
                    vec![
                        ("key".into(), TypeMapField::column("settingKey")),
                        ("value".into(), TypeMapField::column("settingValue")),
                    ],
                )),
            ),
        ],
    ));
    let mut times: HashMap<&str, HashMap<&str, Vec<f64>>> = ["default", "hashset", "flat"]
        .into_iter()
        .map(|mode| (mode, ["ids", "where", "firstName"].into_iter().map(|shape| (shape, Vec::new())).collect()))
        .collect();
    run_reads(contacts_repo, hashset_repo, join_repo, &mapper, sample(&mut state, total, items, 0), related, false).await?;
    for iter in 0..iters {
        let measured = run_reads(
            contacts_repo,
            hashset_repo,
            join_repo,
            &mapper,
            sample(&mut state, total, items, iter),
            related,
            iter == 0,
        )
        .await?;
        for (mode, shapes) in measured {
            for (shape, ms) in shapes {
                times.get_mut(mode).expect("mode").get_mut(shape).expect("shape").push(ms);
            }
        }
    }
    Ok(serde_json::json!({
        "default": summary(&times["default"], items),
        "hashset": summary(&times["hashset"], items),
        "flat": summary(&times["flat"], items),
    }))
}

fn summary(shapes: &HashMap<&str, Vec<f64>>, items: usize) -> serde_json::Value {
    let cell = |shape: &str| {
        let ms = median(&shapes[shape]);
        serde_json::json!({ "ms": ms, "perSec": items as f64 / (ms / 1000.0) })
    };
    serde_json::json!({ "ids": cell("ids"), "where": cell("where"), "firstName": cell("firstName") })
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        sorted[mid]
    } else {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    }
}

fn sample(state: &mut u32, total: usize, items: usize, iter: usize) -> (Vec<i64>, i64) {
    let mut ids: Vec<i64> = (1..=total as i64).collect();
    for i in 0..items {
        let swap = i + ((next(state) * (total - i) as f64) as usize);
        ids.swap(i, swap);
    }
    ids.truncate(items);
    let lo = if items >= total { 0 } else { (iter * items) % (total - items + 1) } as i64;
    (ids, lo)
}

fn next(state: &mut u32) -> f64 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    *state as f64 / 4294967296.0
}

async fn run_reads(
    contacts_repo: &BulkCRUDRepository,
    hashset_repo: &BulkCRUDRepository,
    join_repo: &CRUDRepository,
    mapper: &FlatToNestedMapper,
    sample: (Vec<i64>, i64),
    related: usize,
    check: bool,
) -> Result<HashMap<&'static str, HashMap<&'static str, f64>>> {
    let (ids, lo) = sample;
    let expected = ids.len();
    let mut measured = HashMap::new();
    let ids_where = WhereText::new(format!(
        "id IN ({})",
        ids.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")
    ));
    let range_where = WhereText::new(format!(
        "id > {lo} AND id <= {}",
        lo + expected as i64
    ));
    let name_where = WhereText::new(format!(
        "first_name IN ({})",
        ids.iter().map(|id| format!("'First{id}'")).collect::<Vec<_>>().join(",")
    ));
    for (shape, where_clause) in [
        ("ids", &ids_where),
        ("where", &range_where),
        ("firstName", &name_where),
    ] {
        let default_ms = time(|| async {
            check_enriched(&contacts_repo.read(Some(where_clause), None, None).await?, expected, related, check)
        })
        .await?;
        let hashset_ms = time(|| async {
            check_enriched(&hashset_repo.read(Some(where_clause), None, None).await?, expected, related, check)
        })
        .await?;
        let fields = [
            "Contact.id AS id",
            "Contact.firstName AS firstName",
            "Address.id AS addressId",
            "Phone.id AS phoneId",
            "ContactSettings.key AS settingKey",
            "ContactSettings.value AS settingValue",
        ];
        let joins = [
            Join::left("Address", "Contact.id", "Address.contactId"),
            Join::left("Phone", "Contact.id", "Phone.contactId"),
            Join::left("ContactSettings", "Contact.id", "ContactSettings.contactId"),
        ];
        let join_where = match shape {
            "ids" => WhereText::new(format!(
                "contacts.id IN ({})",
                ids.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")
            )),
            "where" => WhereText::new(format!(
                "contacts.id > {lo} AND contacts.id <= {}",
                lo + expected as i64
            )),
            _ => WhereText::new(format!(
                "contacts.first_name IN ({})",
                ids.iter().map(|id| format!("'First{id}'")).collect::<Vec<_>>().join(",")
            )),
        };
        let mut options = SelectStatementOptions::new("Contact", &fields);
        options.joins = &joins;
        options.where_clause = Some(&join_where);
        let select = SelectStatement::new(options);
        let flat_ms = time(|| async {
            check_mapped(
                &mapper.map(join_repo.read(Some(&select), None, None).await?),
                expected,
                related,
                check,
            )
        })
        .await?;
        measured.insert(
            shape,
            HashMap::from([("default", default_ms), ("hashset", hashset_ms), ("flat", flat_ms)]),
        );
    }
    let mut by_mode = HashMap::new();
    for mode in ["default", "hashset", "flat"] {
        by_mode.insert(
            mode,
            measured
                .iter()
                .map(|(shape, modes)| (*shape, modes[mode]))
                .collect(),
        );
    }
    Ok(by_mode)
}

fn check_enriched(loaded: &[Row], expected: usize, related: usize, check: bool) -> Result<()> {
    if !check {
        return Ok(());
    }
    if loaded.len() != expected {
        return Err(datasources::DatasourceError::message(format!("read count {}", loaded.len())));
    }
    if loaded.iter().any(|row| encoded_count(row, "addresses") != related || encoded_count(row, "phones") != related || encoded_count(row, "settings") != related)
    {
        return Err(datasources::DatasourceError::message("read related"));
    }
    Ok(())
}

fn check_mapped(loaded: &[datasources::MappedRow], expected: usize, related: usize, check: bool) -> Result<()> {
    if !check {
        return Ok(());
    }
    if loaded.len() != expected {
        return Err(datasources::DatasourceError::message(format!("flat count {}", loaded.len())));
    }
    if loaded.iter().any(|row| many_count(row, "addresses") != related || many_count(row, "phones") != related || many_count(row, "settings") != related)
    {
        return Err(datasources::DatasourceError::message("flat related"));
    }
    Ok(())
}

fn encoded_count(row: &Row, name: &str) -> usize {
    match row.get(name) {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').count(),
        _ => 0,
    }
}

fn many_count(row: &datasources::MappedRow, name: &str) -> usize {
    match row.get(name) {
        Some(MappedValue::Many(items)) => items.len(),
        _ => 0,
    }
}

fn env(name: &str, fallback: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| fallback.into())
}

fn shared() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contacts-crud-enriched-medium-rest-shared")
}

fn split_semi(sql: &str) -> Vec<&str> {
    sql.split(';').map(str::trim).filter(|part| !part.is_empty()).collect()
}

fn split_dash(sql: &str) -> Vec<&str> {
    sql.split("---").map(str::trim).filter(|part| !part.is_empty()).collect()
}
