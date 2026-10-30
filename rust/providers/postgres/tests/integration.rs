use super::command::PostgresCommand;
use super::connection::PostgresConnection;
use super::isolate;
use datasources::{
    utc_iso_millis, Command, CommandMiddleware, Connection, ConsoleCommandMiddleware, Native,
    IsolationLevel, Parameter, Reader, Transaction, Value,
};
use serde::Deserialize;
use datasources_tests::specs_dir;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[derive(Deserialize)]
struct Spec {
    name: String,
    connections: Option<Vec<String>>,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    execute: Option<Op>,
    #[serde(rename = "executeReader")]
    execute_reader: Option<Op>,
    #[serde(rename = "startTransaction")]
    start_transaction: Option<Tx>,
    commit: Option<TxEnd>,
    abort: Option<TxEnd>,
    close: Option<serde_yaml::Value>,
}

#[derive(Deserialize)]
struct Tx {
    connection: Option<String>,
    isolation: Option<String>,
    #[serde(rename = "as")]
    alias: Option<String>,
}

#[derive(Deserialize)]
struct TxEnd {
    transaction: Option<String>,
}

#[derive(Deserialize)]
struct Op {
    sql: String,
    connection: Option<String>,
    parameters: Option<BTreeMap<String, serde_yaml::Value>>,
    open: Option<bool>,
    expect: Option<Expect>,
}

#[derive(Deserialize)]
struct Expect {
    #[serde(rename = "rowsAffected")]
    rows_affected: Option<i64>,
    result: Option<serde_yaml::Value>,
    rows: Option<serde_yaml::Value>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct DialectFile {
    #[serde(rename = "createUserTable")]
    create_user_table: String,
    #[serde(rename = "createUserTableVersioned")]
    create_user_table_versioned: String,
    #[serde(rename = "dirtyReadRows")]
    dirty_read_rows: serde_yaml::Value,
}

fn load_dialect() -> DialectFile {
    serde_yaml::from_str(
        &fs::read_to_string(specs_dir().join("dialects/postgres.yaml")).unwrap(),
    )
    .unwrap()
}

fn render_schema(name: &str) -> String {
    let renderer = datasources::StatementTextRenderer::new(
        Some(Box::new(crate::statement_renderer::PostgresStatementRenderer)),
        None,
    );
    datasources_tests::schema_tables(name, |id| id.to_string(), None)
        .iter()
        .map(|statement| renderer.render(statement))
        .collect::<Vec<_>>()
        .join(";\n")
}

fn resolve_sql(sql: &str, dialect: &DialectFile) -> String {
    match sql.trim() {
        "createUserTable" => render_schema(&dialect.create_user_table),
        "createUserTableVersioned" => render_schema(&dialect.create_user_table_versioned),
        _ => sql.to_string(),
    }
}

fn resolve_rows(
    rows: Option<&serde_yaml::Value>,
    dialect: &DialectFile,
) -> Option<Vec<BTreeMap<String, serde_yaml::Value>>> {
    match rows {
        None => None,
        Some(serde_yaml::Value::String(name)) if name == "dirtyReadRows" => {
            serde_yaml::from_value(dialect.dirty_read_rows.clone()).ok()
        }
        Some(value) => serde_yaml::from_value(value.clone()).ok(),
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

fn params(map: &Option<BTreeMap<String, serde_yaml::Value>>) -> Vec<Parameter> {
    map.as_ref()
        .map(|items| {
            items
                .iter()
                .map(|(name, value)| Parameter::new(name.clone(), yaml_value(value)))
                .collect()
        })
        .unwrap_or_default()
}

fn iso_z(text: &str) -> Option<String> {
    match super::converters::field("datetime")
        .and_then(|converter| converter.convert(&Value::Text(text.into())).ok())
    {
        Some(Value::DateTime(date)) => {
            let iso = Value::iso_datetime(date);
            Some(
                iso.strip_suffix(".000Z")
                    .map(|prefix| format!("{prefix}Z"))
                    .unwrap_or(iso),
            )
        }
        _ => None,
    }
}

fn norm(value: &Value) -> serde_yaml::Value {
    match value {
        Value::Null => serde_yaml::Value::Null,
        Value::Bool(flag) => serde_yaml::Value::Bool(*flag),
        Value::Int(number) => serde_yaml::Value::Number((*number).into()),
        Value::Float(number) => serde_yaml::Value::Number(serde_yaml::Number::from(*number)),
        Value::Text(text) => serde_yaml::Value::String(iso_z(text).unwrap_or_else(|| text.clone())),
        Value::DateTime(date) => serde_yaml::Value::String(
            iso_z(&Value::iso_datetime(*date)).unwrap_or_else(|| value.wire_text()),
        ),
        other => serde_yaml::Value::String(other.wire_text()),
    }
}

async fn collect(reader: &dyn Reader) -> Vec<datasources::Row> {
    let mut rows = Vec::new();
    while reader.next().await.unwrap() {
        rows.push(reader.current().unwrap());
    }
    rows
}

struct Session {
    conn: Arc<PostgresConnection>,
    command: PostgresCommand,
}

async fn run(path: &std::path::Path, dialect: &DialectFile, url: &str) {
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let names = spec
        .connections
        .clone()
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["default".into()]);
    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let middleware: Arc<dyn CommandMiddleware> = Arc::new(ConsoleCommandMiddleware::with_sink(
        {
            let logs = logs.clone();
            move |line| logs.lock().unwrap().push(line.to_string())
        },
        || utc_iso_millis(SystemTime::now()),
    ));
    let attach = |command: PostgresCommand| {
        command.add_middleware(middleware.clone());
        command
    };
    let mut sessions = HashMap::new();
    for name in &names {
        let conn = Arc::new(PostgresConnection::new(Some(url.to_string()), None));
        conn.open().await.unwrap();
        sessions.insert(
            name.clone(),
            Session {
                command: attach(PostgresCommand::new(conn.clone())),
                conn,
            },
        );
    }
    let mut txs: HashMap<String, Arc<dyn Transaction>> = HashMap::new();
    for step in &spec.steps {
        if step.close.is_some() {
            sessions[&names[0]].conn.close().await.unwrap();
            continue;
        }
        if let Some(start) = &step.start_transaction {
            let session = &sessions[start.connection.as_deref().unwrap_or(&names[0])];
            let isolation = start.isolation.as_deref().map(IsolationLevel::from_name);
            txs.insert(
                start.alias.clone().unwrap_or_else(|| "default".into()),
                session.conn.start_transaction(isolation).await.unwrap(),
            );
            continue;
        }
        if let Some(commit) = &step.commit {
            txs[commit.transaction.as_deref().unwrap_or("default")]
                .commit()
                .await
                .unwrap();
            continue;
        }
        if let Some(abort) = &step.abort {
            txs[abort.transaction.as_deref().unwrap_or("default")]
                .abort()
                .await
                .unwrap();
            continue;
        }
        let op = step
            .execute
            .as_ref()
            .or(step.execute_reader.as_ref())
            .unwrap_or_else(|| panic!("{}: empty step", spec.name));
        let sql = resolve_sql(&op.sql, dialect);
        let expected_rows = resolve_rows(op.expect.as_ref().and_then(|e| e.rows.as_ref()), dialect);
        let fresh;
        let target = if op.open == Some(false) {
            fresh = attach(PostgresCommand::new(Arc::new(PostgresConnection::new(
                None, None,
            ))));
            &fresh
        } else {
            &sessions[op.connection.as_deref().unwrap_or(&names[0])].command
        };
        let args = params(&op.parameters);
        let expected_error = op.expect.as_ref().and_then(|e| e.error.as_deref());
        if step.execute.is_some() {
            match target.execute_native(&Native::new(&sql), Some(&args)).await {
                Ok(response) => {
                    assert!(expected_error.is_none(), "{}", spec.name);
                    if let Some(rows) = op.expect.as_ref().and_then(|e| e.rows_affected) {
                        assert_eq!(response.rows_affected, rows, "{}", spec.name);
                    }
                    if let Some(result) = op.expect.as_ref().and_then(|e| e.result.as_ref()) {
                        assert_eq!(norm(&response.result), *result, "{}", spec.name);
                    }
                }
                Err(error) => {
                    let expected = expected_error.expect(&format!("{}: {error}", spec.name));
                    assert!(error.to_string().contains(expected), "{error}");
                }
            }
        } else {
            match target.execute_reader_native(&Native::new(&sql), Some(&args)).await {
                Ok(reader) => {
                    assert!(expected_error.is_none(), "{}", spec.name);
                    if let Some(expected) = expected_rows {
                        let actual = collect(reader.as_ref()).await;
                        assert_eq!(actual.len(), expected.len(), "{}", spec.name);
                        for (row, want) in actual.iter().zip(expected) {
                            for (key, value) in want {
                                assert_eq!(norm(row.get(&key).unwrap()), value, "{key}");
                            }
                        }
                    }
                }
                Err(error) => {
                    let expected = expected_error.expect(&format!("{}: {error}", spec.name));
                    assert!(error.to_string().contains(expected), "{error}");
                }
            }
        }
    }
    for session in sessions.values() {
        session.conn.close().await.ok();
    }
}

#[tokio::test]
async fn shared_specs() {
    let dialect = load_dialect();
    for entry in fs::read_dir(specs_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("yaml")
            || path.file_name().and_then(|name| name.to_str()) == Some("schemas.yaml")
        {
            continue;
        }
        let isolated = isolate::create().await;
        run(&path, &dialect, &isolated.url).await;
        isolated.drop().await;
    }
}

async fn read_return_value(command: &dyn Command) {
    use datasources::{Parameter, ReturnStatement};
    command
        .execute_native(
            &Native::new("CREATE FUNCTION add_one(n INT) RETURNS INT LANGUAGE sql AS $$ SELECT n + 1 $$"),
            None,
        )
        .await
        .unwrap();
    let parameters = [Parameter::new("n", 4_i64)];
    let ret = command
        .execute_statement(
            &ReturnStatement::new("add_one", &parameters),
            Some(&parameters),
        )
        .await
        .unwrap();
    assert_eq!(ret.result, Value::Int(5));
}

#[tokio::test]
async fn return_and_output_parameters() {
    use datasources::{ExecuteStoredProcedureStatement, ParameterDirection};
    let isolated = isolate::create().await;
    let conn = Arc::new(PostgresConnection::new(Some(isolated.url.clone()), None));
    conn.open().await.unwrap();
    let command = PostgresCommand::new(conn.clone());
    command
        .execute_native(
            &Native::new("CREATE PROCEDURE bump(INOUT result INT) LANGUAGE plpgsql AS $$ BEGIN result := result + 1; END; $$"),
            None,
        )
        .await
        .unwrap();
    let result = [Parameter::with_direction(
        "result",
        4_i64,
        ParameterDirection::InputOutput,
    )];
    let call = command
        .execute_statement(
            &ExecuteStoredProcedureStatement::new("bump", &result),
            Some(&result),
        )
        .await
        .unwrap();
    assert_eq!(call.parameters.as_ref().unwrap()[0].value, Value::Int(5));
    read_return_value(&command).await;
    conn.close().await.unwrap();
    isolated.drop().await;
}
