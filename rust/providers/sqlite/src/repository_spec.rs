use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::converters::register;
use super::rusqlite_driver::RusqliteDriver;
use super::statement_builder::{SqliteColumn, SqliteStatementBuilder};
use datasources::converters::{mapping, ConvertSchema};
use datasources::{
    Command, CommandMiddleware, Connection, ConsoleCommandMiddleware, Parameter,
    ReadonlyRepository, Repository, Statement, StatementBuilder, TableRepository,
    TableRepositoryBuilder, TableRepositoryBuilderOptions, Value, WhereBuilder,
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
}

#[derive(Deserialize, Default)]
struct Step {
    #[serde(rename = "CREATE")]
    create: Option<RowOp>,
    #[serde(rename = "READ")]
    read: Option<IdOp>,
    #[serde(rename = "QUERY")]
    query: Option<QueryOp>,
    #[serde(rename = "READY")]
    ready: Option<WhereOp>,
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
}

#[derive(Deserialize)]
struct IdOp {
    id: serde_yaml::Value,
}

#[derive(Deserialize, Default)]
struct QueryOp {
    #[serde(rename = "where", default)]
    where_clause: Option<BTreeMap<String, serde_yaml::Value>>,
}

#[derive(Deserialize)]
struct WhereOp {
    #[serde(rename = "where")]
    where_clause: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Deserialize)]
struct UpdateByOp {
    row: BTreeMap<String, serde_yaml::Value>,
    #[serde(rename = "where")]
    where_clause: BTreeMap<String, serde_yaml::Value>,
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

fn sql_type(canonical: &str) -> &'static str {
    match canonical {
        "int" => "INTEGER",
        "uuid" => "UUID",
        "string" => "TEXT",
        "bool" => "BOOLEAN",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "datetime" => "DATETIME",
        "binary" => "BLOB",
        _ => panic!("Unknown field type {canonical}"),
    }
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

fn record(map: &BTreeMap<String, serde_yaml::Value>) -> datasources::Record {
    map.iter()
        .map(|(name, value)| (name.clone(), yaml_value(value)))
        .collect()
}

fn where_of(map: &BTreeMap<String, serde_yaml::Value>) -> WhereBuilder {
    let children: Vec<WhereBuilder> = map
        .iter()
        .map(|(name, value)| WhereBuilder::equals(name.clone(), yaml_value(value)))
        .collect();
    WhereBuilder::and(children)
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
                    &fields_of(&create.row, |col| {
                        !spec.type_spec.assign_identity_on_add || !ids.contains(col)
                    }),
                )
                .get_text()
            } else if let Some(read) = &step.read {
                let where_clause = WhereBuilder::equals("id", yaml_value(&read.id));
                StatementBuilder::create_query(table, &refs, Some(&where_clause)).get_text()
            } else if let Some(query) = &step.query {
                let where_clause = query.where_clause.as_ref().map(where_of);
                StatementBuilder::create_query(
                    table,
                    &refs,
                    where_clause.as_ref().map(|w| w as &dyn datasources::Where),
                )
                .get_text()
            } else if let Some(ready) = &step.ready {
                let where_clause = where_of(&ready.where_clause);
                StatementBuilder::create_query(table, &refs, Some(&where_clause)).get_text()
            } else if let Some(update) = &step.update {
                let fields = fields_of(&update.row, |col| !ids.contains(col));
                let where_clause = WhereBuilder::equals("id", yaml_value(&update.row["id"]));
                StatementBuilder::create_update(table, &fields, Some(&where_clause)).get_text()
            } else if let Some(update_by) = &step.update_by {
                let fields = fields_of(&update_by.row, |col| !ids.contains(col));
                let where_clause = where_of(&update_by.where_clause);
                StatementBuilder::create_update(table, &fields, Some(&where_clause)).get_text()
            } else if let Some(delete) = &step.delete {
                let where_clause = WhereBuilder::equals("id", yaml_value(&delete.id));
                StatementBuilder::create_delete(table, Some(&where_clause)).get_text()
            } else if let Some(delete_by) = &step.delete_by {
                let where_clause = where_of(&delete_by.where_clause);
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
    register();
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
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
            Some(
                &SqliteStatementBuilder::create_table(&spec.type_spec.table, &columns)
                    .unwrap()
                    .get_text(),
            ),
            None,
        )
        .await
        .unwrap();
    let mut mapping = mapping::<SpecUser>("sqlite").unwrap();
    mapping.name = spec.type_spec.table.clone();
    let options = TableRepositoryBuilderOptions {
        connection: conn.clone(),
        ids: spec.type_spec.ids.clone(),
        assign_identity_on_add: spec.type_spec.assign_identity_on_add,
        converters: HashMap::new(),
        mapping: Some(mapping),
    };
    let (commands, converters, mapping, statements, ids, assign) =
        TableRepositoryBuilder::parts(options).unwrap();
    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let middleware: Arc<dyn CommandMiddleware> = Arc::new(ConsoleCommandMiddleware::with_sink(
        {
            let logs = logs.clone();
            move |line| logs.lock().unwrap().push(line.to_string())
        },
        || datasources::utc_iso_millis(SystemTime::now()),
    ));
    commands.get.use_middleware(middleware.clone());
    commands.add.use_middleware(middleware.clone());
    commands.update.use_middleware(middleware.clone());
    commands.delete.use_middleware(middleware.clone());
    let repo = TableRepository::from_parts(commands, converters, mapping, statements, ids, assign);
    for step in &spec.steps {
        if let Some(create) = &step.create {
            repo.add(&record(&create.row)).await.unwrap();
        } else if let Some(read) = &step.read {
            let where_clause = WhereBuilder::equals("id", yaml_value(&read.id));
            repo.get_single_where(&where_clause).await.unwrap();
        } else if let Some(query) = &step.query {
            if let Some(where_clause) = &query.where_clause {
                repo.get_where(&where_of(where_clause)).await.unwrap();
            } else {
                repo.get(None).await.unwrap();
            }
        } else if let Some(ready) = &step.ready {
            repo.get_single_where(&where_of(&ready.where_clause))
                .await
                .unwrap();
        } else if let Some(update) = &step.update {
            repo.update(&record(&update.row), None).await.unwrap();
        } else if let Some(update_by) = &step.update_by {
            let where_clause = where_of(&update_by.where_clause);
            repo.update(&record(&update_by.row), Some(&where_clause))
                .await
                .unwrap();
        } else if let Some(delete) = &step.delete {
            let mut row = datasources::Record::new();
            row.insert("id".into(), yaml_value(&delete.id));
            repo.delete(&row, None).await.unwrap();
        } else if let Some(delete_by) = &step.delete_by {
            let where_clause = where_of(&delete_by.where_clause);
            repo.delete(&datasources::Record::new(), Some(&where_clause))
                .await
                .unwrap();
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
    let actual: Vec<String> = logs.lock().unwrap().iter().map(|line| signature(line)).collect();
    if expected != actual {
        panic!(
            "{}: logging mismatch\n--- expected ---\n{}\n--- actual ---\n{}",
            spec.name,
            expected.join("\n"),
            actual.join("\n")
        );
    }
}

struct SpecUser;

impl ConvertSchema for SpecUser {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("uuid", "UUID"),
            ("username", "TEXT"),
            ("is_enabled", "BOOLEAN"),
            ("balance", "NUMERIC"),
            ("location", "REAL"),
            ("created", "DATETIME"),
            ("version", "BLOB"),
        ]
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
