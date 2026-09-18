use base64::Engine;
use chrono::{DateTime, Utc};
use datasources::converters::RowTypeConverter;
use datasources::{
    Command, CommandMiddleware, Connection, ConsoleCommandMiddleware, DefaultTypeConverters,
    Parameter, ReadonlyRepository, CRUDRepository, Result, Row, Statement, StatementBuilder,
    StatementMapper, Repository, RepositoryBuilder, RepositoryBuilderOptions,
    TransformMapper, TypeConverter, Value, WhereBuilder,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

mod add_extra_contact;
mod as_contacts_view;
mod contact_lookup_ids;
mod contact_lookups;
mod contact_view;
mod contacts_join_query;
mod contacts_parity;
mod contacts_service_of;
mod contacts_view;
mod expect_contacts_view;
mod from_enriched_contacts;
mod from_mapped_contacts;
mod seed_contacts;

pub use add_extra_contact::add_extra_contact;
pub use as_contacts_view::as_contacts_view;
pub use contact_lookup_ids::contact_lookup_ids;
pub use contact_lookups::ContactLookups;
pub use contact_view::{AddressView, ContactView, PhoneView};
pub use contacts_parity::{apply_sql, contact_schema, contacts_parity, statements_of};
pub use contacts_service_of::contacts_service_of;
pub use expect_contacts_view::{contact_with, expect_contacts_view};
pub use seed_contacts::{seed_contacts_sql, SEED_CONTACT_COUNT};

pub struct SpecColumn {
    pub name: String,
    pub type_name: String,
    pub id: bool,
}

impl SpecColumn {
    pub fn new(name: impl Into<String>, type_name: impl Into<String>, id: bool) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            id,
        }
    }
}

pub struct RepositoryDialect {
    pub provider: &'static str,
    pub quote: fn(&str) -> String,
    pub create_table: fn(&str, &[SpecColumn]) -> String,
    pub field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub register: fn(),
}

#[derive(Deserialize)]
struct Spec {
    name: String,
    #[serde(rename = "type")]
    type_spec: TypeSpec,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct TypeSpec {
    table: String,
    ids: Vec<String>,
    #[serde(rename = "assignIdentityOnAdd")]
    assign_identity_on_create: bool,
    fields: BTreeMap<String, String>,
    #[serde(default)]
    converters: BTreeMap<String, String>,
    #[serde(default)]
    mappings: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default, rename = "statementMapper")]
    statement_mapper: Option<StatementMapperSpec>,
}

#[derive(Deserialize, Default)]
struct StatementMapperSpec {
    entity: Option<String>,
    #[serde(default, rename = "querySuffix")]
    query_suffix: Option<String>,
    #[serde(default, rename = "writeSuffix")]
    write_suffix: Option<String>,
}

#[derive(Deserialize, Default)]
struct Step {
    #[serde(rename = "CREATE")]
    create: Option<RowOp>,
    #[serde(rename = "GET")]
    get: Option<IdOp>,
    #[serde(rename = "QUERY")]
    query: Option<QueryOp>,
    #[serde(rename = "GETBY")]
    get_by: Option<WhereOp>,
    #[serde(rename = "UPDATE")]
    update: Option<RowOp>,
    #[serde(rename = "UPDATEBY")]
    update_by: Option<UpdateByOp>,
    #[serde(rename = "DELETE")]
    delete: Option<IdOp>,
    #[serde(rename = "DELETEBY")]
    delete_by: Option<WhereOp>,
}

#[derive(Deserialize)]
struct RowOp {
    row: BTreeMap<String, serde_yaml::Value>,
    #[serde(default)]
    expect: Option<serde_yaml::Value>,
}

#[derive(Deserialize)]
struct IdOp {
    id: serde_yaml::Value,
    #[serde(default)]
    expect: Option<serde_yaml::Value>,
}

#[derive(Deserialize, Default)]
struct QueryOp {
    #[serde(rename = "where", default)]
    where_clause: Option<BTreeMap<String, serde_yaml::Value>>,
    #[serde(default)]
    expect: Option<Vec<BTreeMap<String, serde_yaml::Value>>>,
}

#[derive(Deserialize)]
struct WhereOp {
    #[serde(rename = "where")]
    where_clause: BTreeMap<String, serde_yaml::Value>,
    #[serde(default)]
    expect: Option<serde_yaml::Value>,
}

#[derive(Deserialize)]
struct UpdateByOp {
    row: BTreeMap<String, serde_yaml::Value>,
    #[serde(rename = "where")]
    where_clause: BTreeMap<String, serde_yaml::Value>,
    #[serde(default)]
    expect: Option<serde_yaml::Value>,
}

pub fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

fn verbose() -> bool {
    matches!(std::env::var("VERBOSE").as_deref(), Ok("1") | Ok("true"))
}

fn yaml_value(value: &serde_yaml::Value) -> Value {
    match value {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(flag) => Value::Bool(*flag),
        serde_yaml::Value::Number(number) => number
            .as_i64()
            .map(Value::Int)
            .or_else(|| number.as_f64().map(Value::Float))
            .unwrap_or(Value::Null),
        serde_yaml::Value::String(text) => Value::Text(text.clone()),
        _ => Value::Text(format!("{value:?}")),
    }
}

fn where_sql(map: &BTreeMap<String, serde_yaml::Value>) -> WhereBuilder {
    let children: Vec<WhereBuilder> = map
        .iter()
        .map(|(name, value)| WhereBuilder::equals(name.clone(), yaml_value(value)))
        .collect();
    WhereBuilder::and(children)
}

fn where_of(mapping: &RowTypeConverter, map: &BTreeMap<String, serde_yaml::Value>) -> WhereBuilder {
    let children: Vec<WhereBuilder> = map
        .iter()
        .map(|(name, value)| {
            let raw = yaml_value(value);
            let converted = mapping
                .fields
                .get(name)
                .map(|converter| converter.reverse_convert(&raw).unwrap())
                .unwrap_or(raw);
            WhereBuilder::equals(name.clone(), converted)
        })
        .collect();
    WhereBuilder::and(children)
}

fn fields_overlay(
    type_spec: &TypeSpec,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    let mut fields: HashMap<_, _> = type_spec
        .fields
        .iter()
        .filter_map(|(name, canonical)| field(canonical).map(|converter| (name.clone(), converter)))
        .collect();
    for (name, canonical) in &type_spec.converters {
        if let Some(converter) = field(canonical) {
            fields.insert(name.clone(), converter);
        }
    }
    fields
}

fn mapping_of(spec: &Spec, dialect: &RepositoryDialect) -> RowTypeConverter {
    (dialect.register)();
    let columns: Vec<String> = spec.type_spec.fields.keys().cloned().collect();
    RowTypeConverter::new(columns)
        .with_types(DefaultTypeConverters::for_provider(dialect.provider))
        .with_fields(fields_overlay(&spec.type_spec, dialect.field))
}

fn mappings_mapper(
    mappings: &BTreeMap<String, BTreeMap<String, String>>,
) -> TransformMapper<Row, Row> {
    let nest = mappings.clone();
    let flat = mappings.clone();
    TransformMapper::new(
        move |mut row: Row| {
            for (nested, cols) in &nest {
                if cols.len() != 1 {
                    continue;
                }
                let col = cols.values().next().expect("mapping column");
                if let Some(value) = row.remove(col) {
                    row.insert(nested.clone(), value);
                }
            }
            row
        },
        move |mut row: Row| {
            for (nested, cols) in &flat {
                if cols.len() != 1 {
                    continue;
                }
                let col = cols.values().next().expect("mapping column");
                if let Some(value) = row.remove(nested) {
                    row.insert(col.clone(), value);
                }
            }
            row
        },
    )
}

fn flatten_yaml(
    row: &BTreeMap<String, serde_yaml::Value>,
    mappings: &BTreeMap<String, BTreeMap<String, String>>,
) -> BTreeMap<String, serde_yaml::Value> {
    let mut dest = row.clone();
    for (nested, cols) in mappings {
        let Some(serde_yaml::Value::Mapping(nest)) = dest.get(nested).cloned() else {
            continue;
        };
        for (field, col) in cols {
            if let Some(value) = nest.get(&serde_yaml::Value::String(field.clone())) {
                dest.insert(col.clone(), value.clone());
            }
        }
        dest.remove(nested);
    }
    dest
}

fn domain_record(
    map: &BTreeMap<String, serde_yaml::Value>,
    mappings: &BTreeMap<String, BTreeMap<String, String>>,
) -> Row {
    let mut row = Row::new();
    for (name, value) in map {
        match value {
            serde_yaml::Value::Mapping(nest) => {
                if let Some(cols) = mappings.get(name) {
                    if cols.len() == 1 {
                        let field = cols.keys().next().expect("mapping field");
                        if let Some(inner) = nest.get(&serde_yaml::Value::String(field.clone())) {
                            row.insert(name.clone(), yaml_value(inner));
                        }
                    }
                }
            }
            _ => {
                row.insert(name.clone(), yaml_value(value));
            }
        }
    }
    row
}

fn decimal_text(text: &str) -> Option<String> {
    Decimal::from_str(text)
        .ok()
        .map(|number| number.normalize().to_string())
}

fn cell_of(value: &Value) -> String {
    match value {
        Value::Binary(bytes) => base64::engine::general_purpose::STANDARD.encode(bytes),
        Value::Decimal(number) => number.normalize().to_string(),
        Value::Float(number) => Decimal::from_str(&number.to_string())
            .map(|parsed| parsed.normalize().to_string())
            .unwrap_or_else(|_| number.to_string()),
        Value::Text(text) => DateTime::parse_from_rfc3339(text)
            .map(|parsed| Value::iso_datetime(parsed.with_timezone(&Utc)))
            .ok()
            .or_else(|| decimal_text(text))
            .unwrap_or_else(|| text.clone()),
        other => other.wire_text(),
    }
}

fn expect_cell(value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::Null => String::new(),
        serde_yaml::Value::Bool(flag) => flag.to_string(),
        serde_yaml::Value::Number(number) => {
            decimal_text(&number.to_string()).unwrap_or_else(|| number.to_string())
        }
        serde_yaml::Value::String(text) => DateTime::parse_from_rfc3339(text)
            .map(|parsed| Value::iso_datetime(parsed.with_timezone(&Utc)))
            .ok()
            .or_else(|| decimal_text(text))
            .unwrap_or_else(|| text.clone()),
        serde_yaml::Value::Mapping(map) if map.len() == 1 => {
            expect_cell(map.values().next().expect("nested expect"))
        }
        other => format!("{other:?}"),
    }
}

fn assert_row(name: &str, actual: Option<&Row>, expect: &serde_yaml::Value) {
    match (actual, expect) {
        (None, serde_yaml::Value::Null) => {}
        (Some(row), serde_yaml::Value::Mapping(map)) => {
            for (key, wanted) in map {
                let key = key.as_str().expect("expect key");
                let got = cell_of(row.get(key).unwrap_or(&Value::Null));
                let expected = expect_cell(wanted);
                if got != expected {
                    panic!("{name}: {key}: expected {expected} got {got}");
                }
            }
        }
        (None, _) => panic!("{name}: expected a row"),
        (Some(_), serde_yaml::Value::Null) => panic!("{name}: expected null"),
        _ => panic!("{name}: expect must be a row or null"),
    }
}

fn assert_affected(name: &str, actual: i64, expect: Option<&serde_yaml::Value>) {
    let Some(expect) = expect else {
        return;
    };
    let wanted = expect
        .get("rowsAffected")
        .and_then(serde_yaml::Value::as_i64)
        .unwrap_or_else(|| panic!("{name}: expect.rowsAffected required"));
    if actual != wanted {
        panic!("{name}: expected {wanted} rows affected got {actual}");
    }
}

fn assert_rows(name: &str, actual: &[Row], expect: &[BTreeMap<String, serde_yaml::Value>]) {
    if actual.len() != expect.len() {
        panic!(
            "{name}: expected {} rows got {}",
            expect.len(),
            actual.len()
        );
    }
    for (index, (row, wanted)) in actual.iter().zip(expect).enumerate() {
        let mapping = serde_yaml::Value::Mapping(
            wanted
                .iter()
                .map(|(key, value)| (serde_yaml::Value::String(key.clone()), value.clone()))
                .collect(),
        );
        assert_row(&format!("{name}[{index}]"), Some(row), &mapping);
    }
}

fn fields_of(
    row: &BTreeMap<String, serde_yaml::Value>,
    include: impl Fn(&str) -> bool,
) -> Vec<Parameter> {
    let mut pairs: Vec<_> = row
        .iter()
        .filter(|(key, _)| include(key))
        .map(|(name, value)| (name.clone(), yaml_value(value)))
        .collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
        .into_iter()
        .map(|(name, value)| Parameter::new(name, value))
        .collect()
}

fn columns_of(type_spec: &TypeSpec) -> Vec<String> {
    let mut columns: Vec<String> = type_spec.fields.keys().cloned().collect();
    columns.sort();
    columns
}

fn physical_table(type_spec: &TypeSpec) -> String {
    type_spec
        .statement_mapper
        .as_ref()
        .and_then(|mapper| mapper.entity.clone())
        .unwrap_or_else(|| type_spec.table.clone())
}

fn with_statement_sql(sql: String, write: bool, type_spec: &TypeSpec) -> String {
    let Some(mapper) = &type_spec.statement_mapper else {
        return sql;
    };
    let suffix = if write {
        mapper.write_suffix.as_deref()
    } else {
        mapper.query_suffix.as_deref()
    };
    match suffix {
        Some(suffix) => format!("{sql}{suffix}"),
        None => sql,
    }
}

fn statement_mapper_of(spec: &Spec, dialect: &RepositoryDialect) -> StatementMapper {
    let mut mapper = StatementMapper::identity(dialect.provider, "rust");
    mapper.rows = mappings_mapper(&spec.type_spec.mappings);
    let quote = dialect.quote;
    let Some(cfg) = &spec.type_spec.statement_mapper else {
        mapper.entity = TransformMapper::new(move |name: String| quote(&name), |name| name);
        return mapper;
    };
    if let Some(entity) = &cfg.entity {
        let dest = entity.clone();
        mapper.entity = TransformMapper::new(move |_: String| quote(&dest), |name| name);
    } else {
        mapper.entity = TransformMapper::new(move |name: String| quote(&name), |name| name);
    }
    if let Some(suffix) = &cfg.query_suffix {
        let suffix = suffix.clone();
        mapper.query = Some(Arc::new(TransformMapper::new(
            move |text| format!("{text}{suffix}"),
            |text| text,
        )));
    }
    if let Some(suffix) = &cfg.write_suffix {
        let insert = suffix.clone();
        mapper.insert = Some(Arc::new(TransformMapper::new(
            move |text| format!("{text}{insert}"),
            |text| text,
        )));
        let update = suffix.clone();
        mapper.update = Some(Arc::new(TransformMapper::new(
            move |text| format!("{text}{update}"),
            |text| text,
        )));
        let delete = suffix.clone();
        mapper.delete = Some(Arc::new(TransformMapper::new(
            move |text| format!("{text}{delete}"),
            |text| text,
        )));
    }
    mapper
}

fn queries(spec: &Spec) -> Vec<String> {
    let columns = columns_of(&spec.type_spec);
    let refs: Vec<&str> = columns.iter().map(String::as_str).collect();
    let ids: HashSet<&str> = spec.type_spec.ids.iter().map(String::as_str).collect();
    let table = physical_table(&spec.type_spec);
    spec.steps
        .iter()
        .map(|step| {
            if let Some(create) = &step.create {
                with_statement_sql(
                    StatementBuilder::create_insert(
                        &table,
                        &fields_of(
                            &flatten_yaml(&create.row, &spec.type_spec.mappings),
                            |col| !spec.type_spec.assign_identity_on_create || !ids.contains(col),
                        ),
                    )
                    .get_text(),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(get) = &step.get {
                let where_clause = WhereBuilder::equals("id", yaml_value(&get.id));
                with_statement_sql(
                    StatementBuilder::create_query(&table, &refs, Some(&where_clause), &[])
                        .get_text(),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(query) = &step.query {
                let where_clause = query.where_clause.as_ref().map(where_sql);
                with_statement_sql(
                    StatementBuilder::create_query(
                        &table,
                        &refs,
                        where_clause.as_ref().map(|w| w as &dyn datasources::Where),
                        &[],
                    )
                    .get_text(),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(get_by) = &step.get_by {
                let where_clause = where_sql(&get_by.where_clause);
                with_statement_sql(
                    StatementBuilder::create_query(&table, &refs, Some(&where_clause), &[])
                        .get_text(),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(update) = &step.update {
                let flat = flatten_yaml(&update.row, &spec.type_spec.mappings);
                let fields = fields_of(&flat, |col| !ids.contains(col));
                let where_clause = WhereBuilder::equals("id", yaml_value(&flat["id"]));
                with_statement_sql(
                    StatementBuilder::create_update(&table, &fields, Some(&where_clause))
                        .get_text(),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(update_by) = &step.update_by {
                let fields = fields_of(
                    &flatten_yaml(&update_by.row, &spec.type_spec.mappings),
                    |col| !ids.contains(col),
                );
                let where_clause = where_sql(&update_by.where_clause);
                with_statement_sql(
                    StatementBuilder::create_update(&table, &fields, Some(&where_clause))
                        .get_text(),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(delete) = &step.delete {
                let where_clause = WhereBuilder::equals("id", yaml_value(&delete.id));
                with_statement_sql(
                    StatementBuilder::create_delete(&table, Some(&where_clause)).get_text(),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(delete_by) = &step.delete_by {
                let where_clause = where_sql(&delete_by.where_clause);
                with_statement_sql(
                    StatementBuilder::create_delete(&table, Some(&where_clause)).get_text(),
                    true,
                    &spec.type_spec,
                )
            } else {
                panic!("{}: empty step", spec.name);
            }
        })
        .collect()
}

fn unquote_sql(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '`' || ch == '"' {
            let close = ch;
            while let Some(inner) = chars.next() {
                if inner == close {
                    break;
                }
                out.push(inner);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn signature(line: &str) -> String {
    let body = line
        .find("]-")
        .map(|index| &line[index + 2..])
        .unwrap_or(line);
    let body = if body.ends_with("ms") || body.ends_with("μs") {
        if let Some(split) = body.rfind("] ") {
            body[..=split].to_string()
        } else {
            body.to_string()
        }
    } else {
        body.to_string()
    };
    unquote_sql(&body)
}

pub async fn run(
    path: &Path,
    dialect: &RepositoryDialect,
    connection: Arc<dyn Connection>,
    command: Arc<dyn Command>,
) -> Result<()> {
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    if verbose() {
        println!("\n--- {} ({}) ---", spec.name, dialect.provider);
    }
    let columns: Vec<SpecColumn> = spec
        .type_spec
        .fields
        .iter()
        .map(|(name, canonical)| {
            SpecColumn::new(
                name.clone(),
                canonical.clone(),
                spec.type_spec.ids.contains(name),
            )
        })
        .collect();
    command
        .execute(
            &(dialect.create_table)(&physical_table(&spec.type_spec), &columns),
            None,
        )
        .await?;
    let mapping = mapping_of(&spec, dialect);
    let options = RepositoryBuilderOptions {
        connection,
        ids: spec.type_spec.ids.clone(),
        assign_identity_on_create: spec.type_spec.assign_identity_on_create,
        entity: spec.type_spec.table.clone(),
        mapping: Some(mapping_of(&spec, dialect)),
        mapper: None,
        statement_mapper: Some(statement_mapper_of(&spec, dialect)),
        types: Default::default(),
        fields: Default::default(),
    };
    let built = RepositoryBuilder::parts(options).unwrap();
    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let middleware: Arc<dyn CommandMiddleware> = Arc::new(
        ConsoleCommandMiddleware::with_sink(
            {
                let logs = logs.clone();
                move |line| {
                    if verbose() {
                        println!("{line}");
                    }
                    logs.lock().unwrap().push(line.to_string())
                }
            },
            || datasources::utc_iso_millis(SystemTime::now()),
        )
        .options(HashMap::from([(
            "isVerbose".into(),
            if verbose() { "true" } else { "false" }.into(),
        )])),
    );
    let mut repo = Repository::from_built(built);
    repo.add_middleware(middleware);
    for step in &spec.steps {
        match step {
            Step {
                create: Some(create),
                ..
            } => {
                repo.create(&domain_record(&create.row, &spec.type_spec.mappings))
                    .await
                    .unwrap();
            }
            Step { get: Some(get), .. } => {
                let where_clause = WhereBuilder::equals("id", yaml_value(&get.id));
                let row = repo.get_single_where(&where_clause).await.unwrap();
                if let Some(expect) = &get.expect {
                    assert_row(&format!("{} GET", spec.name), row.as_ref(), expect);
                }
            }
            Step {
                query: Some(query), ..
            } => {
                let rows = if let Some(where_clause) = &query.where_clause {
                    repo.get_where(&where_of(&mapping, where_clause))
                        .await
                        .unwrap()
                } else {
                    repo.get(None).await.unwrap()
                };
                if let Some(expect) = &query.expect {
                    assert_rows(&format!("{} QUERY", spec.name), &rows, expect);
                }
            }
            Step {
                get_by: Some(get_by),
                ..
            } => {
                let row = repo
                    .get_single_where(&where_of(&mapping, &get_by.where_clause))
                    .await
                    .unwrap();
                if let Some(expect) = &get_by.expect {
                    assert_row(&format!("{} GETBY", spec.name), row.as_ref(), expect);
                }
            }
            Step {
                update: Some(update),
                ..
            } => {
                let response = repo
                    .update(&domain_record(&update.row, &spec.type_spec.mappings), None)
                    .await
                    .unwrap();
                assert_affected(
                    &format!("{} UPDATE", spec.name),
                    response.rows_affected,
                    update.expect.as_ref(),
                );
            }
            Step {
                update_by: Some(update_by),
                ..
            } => {
                let where_clause = where_of(&mapping, &update_by.where_clause);
                let response = repo
                    .update(
                        &domain_record(&update_by.row, &spec.type_spec.mappings),
                        Some(&where_clause),
                    )
                    .await
                    .unwrap();
                assert_affected(
                    &format!("{} UPDATEBY", spec.name),
                    response.rows_affected,
                    update_by.expect.as_ref(),
                );
            }
            Step {
                delete: Some(delete),
                ..
            } => {
                let mut row = Row::new();
                row.insert("id".into(), yaml_value(&delete.id));
                let response = repo.delete(&row, None).await.unwrap();
                assert_affected(
                    &format!("{} DELETE", spec.name),
                    response.rows_affected,
                    delete.expect.as_ref(),
                );
            }
            Step {
                delete_by: Some(delete_by),
                ..
            } => {
                let where_clause = where_of(&mapping, &delete_by.where_clause);
                let response = repo.delete(&Row::new(), Some(&where_clause)).await.unwrap();
                assert_affected(
                    &format!("{} DELETEBY", spec.name),
                    response.rows_affected,
                    delete_by.expect.as_ref(),
                );
            }
            _ => panic!("{}: empty step", spec.name),
        }
    }
    let expected: Vec<String> = queries(&spec)
        .into_iter()
        .flat_map(|query| {
            [
                format!("[datasource][Start]-[{query}]"),
                format!("[datasource][Finish]-[{query}]"),
            ]
        })
        .collect();
    let actual: Vec<String> = logs
        .lock()
        .unwrap()
        .iter()
        .filter(|line| line.contains("[Start]") || line.contains("[Finish]"))
        .map(|line| signature(line))
        .collect();
    if expected != actual {
        panic!(
            "{}: logging mismatch\n--- expected ---\n{}\n--- actual ---\n{}",
            spec.name,
            expected.join("\n"),
            actual.join("\n")
        );
    }
    Ok(())
}

pub fn files() -> Vec<std::path::PathBuf> {
    fs::read_dir(repo_root().join("specs/repositories"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("yaml"))
        .collect()
}
