use futures_util::TryStreamExt;
use base64::Engine;
use chrono::{DateTime, Utc};
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Command, CommandMiddleware, Connection,
    Schema,
    ConsoleCommandMiddleware, DeleteStatement, Expression, ICRUDRepository, IReadonlyRepository,
    InsertStatement, Literal, Operator, Parameter, Result, Row, SelectStatement,
    SelectStatementOptions, DefaultStatementTextRenderer, Statement, TransformMapper, TypeConverter,
    UpdateStatement, Value, Where,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::specs_dir;
use crate::schema_spec::tables as schema_tables;

pub struct RepositoryDialect {
    pub provider: &'static str,
    pub quote: fn(&str) -> String,
    pub field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub types: fn() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    pub render: Option<fn(&dyn Statement) -> String>,
}

#[derive(Deserialize)]
struct Spec {
    name: String,
    schema: String,
    #[serde(rename = "type")]
    type_spec: TypeSpec,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct TypeSpec {
    table: String,
    ids: Vec<String>,
    fields: BTreeMap<String, String>,
    #[serde(default)]
    converters: BTreeMap<String, String>,
    #[serde(default)]
    mappings: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(default, rename = "crudMapper")]
    crud_mapper: Option<CRUDMapperSpec>,
}

#[derive(Deserialize, Default)]
#[allow(dead_code)]
struct CRUDMapperSpec {
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
    #[serde(rename = "READ")]
    read: Option<IdOp>,
    #[serde(rename = "QUERY")]
    query: Option<QueryOp>,
    #[serde(rename = "READBY")]
    read_by: Option<WhereOp>,
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

fn and_all(parts: Vec<Expression>) -> Expression {
    parts
        .into_iter()
        .reduce(|left, right| Operator::and(left, right).into())
        .unwrap_or_else(|| {
            Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }
            .into()
        })
}

fn where_sql(
    dialect: &RepositoryDialect,
    type_spec: &TypeSpec,
    map: &BTreeMap<String, serde_yaml::Value>,
) -> Where {
    let overlay = fields_overlay(type_spec, dialect.field);
    Where::new(and_all(
        map.iter()
            .map(|(name, value)| {
                let raw = yaml_value(value);
                let converted = overlay
                    .get(name)
                    .and_then(|converter| converter.reverse_convert(&raw).ok())
                    .unwrap_or(raw);
                Operator::equals(name.clone(), converted).into()
            })
            .collect(),
    ))
}

fn where_of(mapping: &RowTypeConverter, map: &BTreeMap<String, serde_yaml::Value>) -> Where {
    Where::new(and_all(
        map.iter()
            .map(|(name, value)| {
                let raw = yaml_value(value);
                let converted = mapping
                    .fields
                    .get(name)
                    .map(|converter| converter.reverse_convert(&raw).unwrap())
                    .unwrap_or(raw);
                Operator::equals(name.clone(), converted).into()
            })
            .collect(),
    ))
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
    let columns: HashMap<String, String> = spec.type_spec.fields.clone().into_iter().collect();
    let mut fields = RowTypeConverter::from(|| (dialect.types)(), &columns).fields;
    fields.extend(fields_overlay(&spec.type_spec, dialect.field));
    RowTypeConverter::new(fields)
}

fn assigned_yaml(value: Option<&serde_yaml::Value>) -> bool {
    match value {
        None | Some(serde_yaml::Value::Null) => false,
        Some(serde_yaml::Value::String(text)) if text.is_empty() => false,
        Some(serde_yaml::Value::Number(number))
            if number.as_i64() == Some(0) || number.as_u64() == Some(0) =>
        {
            false
        }
        _ => true,
    }
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
    flatten_yaml(map, mappings)
        .into_iter()
        .map(|(name, value)| (name, yaml_value(&value)))
        .collect()
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

fn assert_row(
    name: &str,
    actual: Option<&Row>,
    expect: &serde_yaml::Value,
    mappings: &BTreeMap<String, BTreeMap<String, String>>,
) {
    match (actual, expect) {
        (None, serde_yaml::Value::Null) => {}
        (Some(row), serde_yaml::Value::Mapping(map)) => {
            let nested = map
                .iter()
                .map(|(key, value)| {
                    (
                        key.as_str().expect("expect key").to_string(),
                        value.clone(),
                    )
                })
                .collect();
            for (key, wanted) in flatten_yaml(&nested, mappings) {
                let got = cell_of(row.get(&key).unwrap_or(&Value::Null));
                let expected = expect_cell(&wanted);
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

fn assert_rows(
    name: &str,
    actual: &[Row],
    expect: &[BTreeMap<String, serde_yaml::Value>],
    mappings: &BTreeMap<String, BTreeMap<String, String>>,
) {
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
        assert_row(&format!("{name}[{index}]"), Some(row), &mapping, mappings);
    }
}

fn convert_params(
    dialect: &RepositoryDialect,
    type_spec: &TypeSpec,
    fields: Vec<Parameter>,
) -> Vec<Parameter> {
    let overlay = fields_overlay(type_spec, dialect.field);
    fields
        .into_iter()
        .map(|parameter| match overlay.get(&parameter.name) {
            Some(converter) => Parameter::new(
                parameter.name,
                converter
                    .reverse_convert(&parameter.value)
                    .unwrap_or(parameter.value),
            ),
            None => parameter,
        })
        .collect()
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
        .crud_mapper
        .as_ref()
        .and_then(|mapper| mapper.entity.clone())
        .unwrap_or_else(|| type_spec.table.clone())
}

fn with_statement_sql(sql: String, _write: bool, _type_spec: &TypeSpec) -> String {
    sql
}

fn statement_sql(dialect: &RepositoryDialect, statement: &dyn Statement) -> String {
    match dialect.render {
        Some(render) => render(statement),
        None => DefaultStatementTextRenderer::instance().render(statement),
    }
}

fn apply_mappers(options: &mut CRUDRepositoryOptions, spec: &Spec, dialect: &RepositoryDialect) {
    let quote = dialect.quote;
    let Some(cfg) = &spec.type_spec.crud_mapper else {
        options.entity_mapper = Some(std::sync::Arc::new(TransformMapper::new(
            move |name: String| quote(&name),
            |name| name,
        )));
        return;
    };
    if let Some(entity) = &cfg.entity {
        let dest = entity.clone();
        options.entity_mapper = Some(std::sync::Arc::new(TransformMapper::new(
            move |_: String| quote(&dest),
            |name| name,
        )));
    } else {
        options.entity_mapper = Some(std::sync::Arc::new(TransformMapper::new(
            move |name: String| quote(&name),
            |name| name,
        )));
    }
}

fn queries(spec: &Spec, dialect: &RepositoryDialect) -> Vec<String> {
    let columns = columns_of(&spec.type_spec);
    let refs: Vec<&str> = columns.iter().map(String::as_str).collect();
    let ids: HashSet<&str> = spec.type_spec.ids.iter().map(String::as_str).collect();
    let table = physical_table(&spec.type_spec);
    spec.steps
        .iter()
        .flat_map(|step| {
            let sql = if let Some(create) = &step.create {
                let flat = flatten_yaml(&create.row, &spec.type_spec.mappings);
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &InsertStatement::new(
                            &table,
                            &convert_params(
                                dialect,
                                &spec.type_spec,
                                fields_of(
                                    &flat,
                                    |col| !ids.contains(col) || assigned_yaml(flat.get(col)),
                                ),
                            ),
                        ),
                    ),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(read) = &step.read {
                let where_clause = Where::new(Operator::equals("id", yaml_value(&read.id)));
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &SelectStatement::new(SelectStatementOptions {
                            where_clause: Some(&where_clause),
                            ..SelectStatementOptions::new(&table, &refs)
                        }),
                    ),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(query) = &step.query {
                let where_clause = query
                    .where_clause
                    .as_ref()
                    .map(|map| where_sql(dialect, &spec.type_spec, map));
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &SelectStatement::new(SelectStatementOptions {
                            where_clause: where_clause.as_ref().map(|w| w as &dyn datasources::IWhere),
                            ..SelectStatementOptions::new(&table, &refs)
                        }),
                    ),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(read_by) = &step.read_by {
                let where_clause = where_sql(dialect, &spec.type_spec, &read_by.where_clause);
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &SelectStatement::new(SelectStatementOptions {
                            where_clause: Some(&where_clause),
                            ..SelectStatementOptions::new(&table, &refs)
                        }),
                    ),
                    false,
                    &spec.type_spec,
                )
            } else if let Some(update) = &step.update {
                let flat = flatten_yaml(&update.row, &spec.type_spec.mappings);
                let fields = convert_params(
                    dialect,
                    &spec.type_spec,
                    fields_of(&flat, |col| !ids.contains(col)),
                );
                let where_clause = Where::new(Operator::equals("id", yaml_value(&flat["id"])));
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &UpdateStatement::new(&table, &fields, Some(&where_clause)),
                    ),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(update_by) = &step.update_by {
                let fields = convert_params(
                    dialect,
                    &spec.type_spec,
                    fields_of(
                        &flatten_yaml(&update_by.row, &spec.type_spec.mappings),
                        |col| !ids.contains(col),
                    ),
                );
                let where_clause = where_sql(dialect, &spec.type_spec, &update_by.where_clause);
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &UpdateStatement::new(&table, &fields, Some(&where_clause)),
                    ),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(delete) = &step.delete {
                let where_clause = Where::new(Operator::equals("id", yaml_value(&delete.id)));
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &DeleteStatement::new(&table, Some(&where_clause)),
                    ),
                    true,
                    &spec.type_spec,
                )
            } else if let Some(delete_by) = &step.delete_by {
                let where_clause = where_sql(dialect, &spec.type_spec, &delete_by.where_clause);
                with_statement_sql(
                    statement_sql(
                        dialect,
                        &DeleteStatement::new(&table, Some(&where_clause)),
                    ),
                    true,
                    &spec.type_spec,
                )
            } else {
                panic!("{}: empty step", spec.name);
            };
            let mut out = vec![sql];
            if let Some(create) = &step.create {
                if spec.type_spec.ids.len() == 1 {
                    if let Some(column) = spec.type_spec.ids.first() {
                        let flat = flatten_yaml(&create.row, &spec.type_spec.mappings);
                        if !assigned_yaml(flat.get(column)) {
                            let inserted = inserted_id_sql(dialect.provider, &table, column);
                            if !inserted.is_empty() {
                                out.push(inserted);
                            }
                        }
                    }
                }
            }
            out
        })
        .collect()
}

fn inserted_id_sql(provider: &str, table: &str, column: &str) -> String {
    match provider {
        "sqlite" => format!("SELECT last_insert_rowid() AS {column}"),
        "mysql" => format!("SELECT LAST_INSERT_ID() AS {column}"),
        "postgres" => format!(
            "SELECT setval(pg_get_serial_sequence('{table}', '{column}'), COALESCE((SELECT MAX(\"{column}\") FROM {table}), 1)) AS {column}"
        ),
        _ => String::new(),
    }
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
    let physical = physical_table(&spec.type_spec);
    let mut rename = HashMap::new();
    if physical != spec.type_spec.table {
        rename.insert(spec.type_spec.table.clone(), physical);
    }
    for statement in schema_tables(
        &spec.schema,
        dialect.quote,
        if rename.is_empty() {
            None
        } else {
            Some(&rename)
        },
    ) {
        command.execute_statement(&statement, None).await?;
    }
    let mapping = mapping_of(&spec, dialect);
    let mut options = CRUDRepositoryOptions::default();
    options.ids = spec.type_spec.ids.clone();
    options.entity = spec.type_spec.table.clone();
    options.schema = Schema::new(spec.type_spec.fields.clone().into_iter().collect());
    options.row_converter = Some(mapping_of(&spec, dialect));
    apply_mappers(&mut options, &spec, dialect);
    let repo = CRUDRepository::<Row>::new(connection, options).unwrap();
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
    repo.add_middleware(middleware);
    for step in &spec.steps {
        match step {
            Step {
                create: Some(create),
                ..
            } => {
                repo.create(domain_record(&create.row, &spec.type_spec.mappings))
                    .await
                    .unwrap();
            }
            Step {
                read: Some(read), ..
            } => {
                let where_clause = Where::new(Operator::equals("id", yaml_value(&read.id)));
                let row = repo.read_single(&where_clause).await.unwrap();
                if let Some(expect) = &read.expect {
                    assert_row(
                        &format!("{} READ", spec.name),
                        row.as_ref(),
                        expect,
                        &spec.type_spec.mappings,
                    );
                }
            }
            Step {
                query: Some(query), ..
            } => {
                let rows = if let Some(where_clause) = &query.where_clause {
                    repo.read(&where_of(&mapping, where_clause))
                        .try_collect::<Vec<_>>().await
                        .unwrap()
                } else {
                    repo.read(&()).try_collect::<Vec<_>>().await.unwrap()
                };
                if let Some(expect) = &query.expect {
                    assert_rows(
                        &format!("{} QUERY", spec.name),
                        &rows,
                        expect,
                        &spec.type_spec.mappings,
                    );
                }
            }
            Step {
                read_by: Some(read_by),
                ..
            } => {
                let row = repo
                    .read_single(&where_of(&mapping, &read_by.where_clause))
                    .await
                    .unwrap();
                if let Some(expect) = &read_by.expect {
                    assert_row(
                        &format!("{} READBY", spec.name),
                        row.as_ref(),
                        expect,
                        &spec.type_spec.mappings,
                    );
                }
            }
            Step {
                update: Some(update),
                ..
            } => {
                let updated_row_1 = domain_record(&update.row, &spec.type_spec.mappings);
                let response = repo
                    .update(updated_row_1)
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
                let updated_row_2 = domain_record(&update_by.row, &spec.type_spec.mappings);
                let by_where = where_of(&mapping, &update_by.where_clause);
                let response = ICRUDRepository::update(&repo, updated_row_2, Some(&by_where))
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
                let response = repo.delete(row).await.unwrap();
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
                let by_where = where_of(&mapping, &delete_by.where_clause);
                let response = ICRUDRepository::delete(&repo, Row::new(), Some(&by_where))
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
    let expected: Vec<String> = queries(&spec, dialect)
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
    fs::read_dir(specs_dir().join("repositories"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("yaml"))
        .collect()
}
