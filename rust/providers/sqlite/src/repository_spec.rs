use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::rusqlite_driver::RusqliteDriver;
use super::statement_builder::{SqliteColumn, SqliteStatementBuilder};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;
use datasources::converters::RowTypeConverter;
use datasources::{
    Command, CommandMiddleware, Connection, ConsoleCommandMiddleware, DefaultTypeConverters,
    Parameter, ReadonlyRepository, Repository, Row, Statement, StatementBuilder, TableRepository,
    TableRepositoryBuilder, TableRepositoryBuilderOptions, TransformMapper, Value, WhereBuilder,
};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

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
    assign_identity_on_add: bool,
    fields: BTreeMap<String, String>,
    #[serde(default)]
    converters: BTreeMap<String, String>,
    #[serde(default)]
    mappings: BTreeMap<String, BTreeMap<String, String>>,
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

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

fn verbose() -> bool {
    matches!(
        std::env::var("VERBOSE").as_deref(),
        Ok("1") | Ok("true")
    )
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

fn where_of(
    mapping: &RowTypeConverter,
    map: &BTreeMap<String, serde_yaml::Value>,
) -> WhereBuilder {
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

fn fields_overlay(type_spec: &TypeSpec) -> HashMap<String, Box<dyn datasources::TypeConverter<Value, Value> + Send + Sync>> {
    let mut fields: HashMap<_, _> = type_spec
        .fields
        .iter()
        .filter_map(|(name, canonical)| {
            crate::converters::field(canonical).map(|converter| (name.clone(), converter))
        })
        .collect();
    for (name, canonical) in &type_spec.converters {
        if let Some(converter) = crate::converters::field(canonical) {
            fields.insert(name.clone(), converter);
        }
    }
    fields
}

fn mapping_of(spec: &Spec) -> RowTypeConverter {
    crate::converters::register();
    let columns: Vec<String> = spec.type_spec.fields.keys().cloned().collect();
    RowTypeConverter::new(columns)
        .with_types(DefaultTypeConverters::for_provider("sqlite"))
        .with_fields(fields_overlay(&spec.type_spec))
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
    Decimal::from_str(text).ok().map(|number| number.normalize().to_string())
}

fn cell_of(value: &Value) -> String {
    match value {
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
        serde_yaml::Value::Number(number) => decimal_text(&number.to_string())
            .unwrap_or_else(|| number.to_string()),
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
        panic!("{name}: expected {} rows got {}", expect.len(), actual.len());
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

fn queries(spec: &Spec) -> Vec<String> {
    let columns = columns_of(&spec.type_spec);
    let refs: Vec<&str> = columns.iter().map(String::as_str).collect();
    let ids: HashSet<&str> = spec.type_spec.ids.iter().map(String::as_str).collect();
    let table = &spec.type_spec.table;
    spec.steps
        .iter()
        .map(|step| {
            if let Some(create) = &step.create {
                StatementBuilder::create_insert(
                    table,
                    &fields_of(
                        &flatten_yaml(&create.row, &spec.type_spec.mappings),
                        |col| !spec.type_spec.assign_identity_on_add || !ids.contains(col),
                    ),
                )
                .get_text()
            } else if let Some(get) = &step.get {
                let where_clause = WhereBuilder::equals("id", yaml_value(&get.id));
                StatementBuilder::create_query(table, &refs, Some(&where_clause)).get_text()
            } else if let Some(query) = &step.query {
                let where_clause = query.where_clause.as_ref().map(where_sql);
                StatementBuilder::create_query(
                    table,
                    &refs,
                    where_clause.as_ref().map(|w| w as &dyn datasources::Where),
                )
                .get_text()
            } else if let Some(get_by) = &step.get_by {
                let where_clause = where_sql(&get_by.where_clause);
                StatementBuilder::create_query(table, &refs, Some(&where_clause)).get_text()
            } else if let Some(update) = &step.update {
                let flat = flatten_yaml(&update.row, &spec.type_spec.mappings);
                let fields = fields_of(&flat, |col| !ids.contains(col));
                let where_clause = WhereBuilder::equals("id", yaml_value(&flat["id"]));
                StatementBuilder::create_update(table, &fields, Some(&where_clause)).get_text()
            } else if let Some(update_by) = &step.update_by {
                let fields = fields_of(
                    &flatten_yaml(&update_by.row, &spec.type_spec.mappings),
                    |col| !ids.contains(col),
                );
                let where_clause = where_sql(&update_by.where_clause);
                StatementBuilder::create_update(table, &fields, Some(&where_clause)).get_text()
            } else if let Some(delete) = &step.delete {
                let where_clause = WhereBuilder::equals("id", yaml_value(&delete.id));
                StatementBuilder::create_delete(table, Some(&where_clause)).get_text()
            } else if let Some(delete_by) = &step.delete_by {
                let where_clause = where_sql(&delete_by.where_clause);
                StatementBuilder::create_delete(table, Some(&where_clause)).get_text()
            } else {
                panic!("{}: empty step", spec.name);
            }
        })
        .collect()
}

fn signature(line: &str) -> String {
    let body = line
        .find("]-")
        .map(|index| &line[index + 2..])
        .unwrap_or(line);
    if body.ends_with("ms") || body.ends_with("μs") {
        if let Some(split) = body.rfind("] ") {
            return body[..=split].to_string();
        }
    }
    body.to_string()
}

async fn run(path: &Path) {
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    if verbose() {
        println!("\n--- {} (sqlite) ---", spec.name);
    }
    let conn = Arc::new(SqliteConnection::new(
        Some(":memory:".into()),
        Arc::new(RusqliteDriver::new()),
    ));
    conn.open().await.unwrap();
    let setup = SqliteCommand::new(conn.clone());
    let columns: Vec<SqliteColumn> = spec
        .type_spec
        .fields
        .iter()
        .map(|(name, canonical)| {
            SqliteColumn::new(name.clone(), canonical.clone(), spec.type_spec.ids.contains(name))
        })
        .collect();
    setup
        .execute(
            &SqliteStatementBuilder::create_table(&spec.type_spec.table, &columns)
                .unwrap()
                .get_text(),
            None,
        )
        .await
        .unwrap();
    let mapping = mapping_of(&spec);
    let options = TableRepositoryBuilderOptions {
        connection: conn.clone(),
        ids: spec.type_spec.ids.clone(),
        assign_identity_on_add: spec.type_spec.assign_identity_on_add,
        entity: spec.type_spec.table.clone(),
        mapping: Some(mapping_of(&spec)),
        mapper: Some(mappings_mapper(&spec.type_spec.mappings)),
        statement_mapper: None,
        types: Default::default(),
        fields: Default::default(),
    };
    let built = TableRepositoryBuilder::parts(options).unwrap();
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
    let mut repo = TableRepository::from_built(built);
    repo.use_middleware(middleware);
    for step in &spec.steps {
        match step {
            Step {
                create: Some(create),
                ..
            } => {
                repo.add(&domain_record(&create.row, &spec.type_spec.mappings))
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
                query: Some(query),
                ..
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
                let response = repo
                    .delete(&Row::new(), Some(&where_clause))
                    .await
                    .unwrap();
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
}

#[tokio::test]
async fn user_crud() {
    let dir = repo_root().join("specs/repositories");
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            run(&path).await;
        }
    }
}
