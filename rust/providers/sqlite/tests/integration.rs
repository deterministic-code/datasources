use super::command::SqliteCommand;
use super::connection::SqliteConnection;
use super::rusqlite_driver::RusqliteDriver;
use datasources::{
    utc_iso_millis, Command, CommandMiddleware, Connection, ConsoleCommandMiddleware,
    IsolationLevel, Parameter, Reader, Transaction, Value,
};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;
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
    connection: DialectConnection,
    #[serde(rename = "createUserTable")]
    create_user_table: String,
    #[serde(rename = "createUserTableVersioned")]
    create_user_table_versioned: String,
    #[serde(rename = "dirtyReadRows")]
    dirty_read_rows: serde_yaml::Value,
}

#[derive(Deserialize)]
struct DialectConnection {
    kind: String,
    filename: Option<String>,
    env: Option<String>,
}

fn shared(filename: &str, connections: usize) -> String {
    if connections <= 1 {
        return filename.to_string();
    }
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ds-{:032x}.db",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    path.to_string_lossy().into_owned()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

fn specs_dir() -> PathBuf {
    repo_root().join("specs")
}

fn load_dialect(engine: &str) -> DialectFile {
    let path = repo_root().join(format!("specs/dialects/{engine}.yaml"));
    serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn resolve_sql(sql: &str, dialect: &DialectFile) -> String {
    match sql.trim() {
        "createUserTable" => dialect.create_user_table.clone(),
        "createUserTableVersioned" => dialect.create_user_table_versioned.clone(),
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

fn dialect_target(dialect: &DialectFile, connections: usize) -> String {
    if dialect.connection.kind == "url" {
        let env = dialect.connection.env.as_deref().unwrap_or("");
        return std::env::var(env).unwrap_or_else(|_| panic!("{env} is not set"));
    }
    shared(
        dialect.connection.filename.as_deref().unwrap_or(":memory:"),
        connections,
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

fn norm(value: &Value) -> serde_yaml::Value {
    match value {
        Value::Null => serde_yaml::Value::Null,
        Value::Bool(flag) => serde_yaml::Value::Bool(*flag),
        Value::Int(number) => serde_yaml::Value::Number((*number).into()),
        Value::Float(number) => serde_yaml::Value::Number(serde_yaml::Number::from(*number)),
        Value::Text(text) => serde_yaml::Value::String(text.clone()),
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
    conn: Arc<SqliteConnection>,
    command: SqliteCommand,
}

async fn run(path: &std::path::Path, dialect: &DialectFile) {
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let names = spec
        .connections
        .clone()
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["default".into()]);
    let filename = dialect_target(dialect, names.len());
    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let middleware: Arc<dyn CommandMiddleware> = Arc::new(ConsoleCommandMiddleware::with_sink(
        {
            let logs = logs.clone();
            move |line| logs.lock().unwrap().push(line.to_string())
        },
        || utc_iso_millis(SystemTime::now()),
    ));
    let attach = |command: SqliteCommand| {
        command.add_middleware(middleware.clone());
        command
    };
    let mut sessions = HashMap::new();
    for name in &names {
        let conn = Arc::new(SqliteConnection::new(
            Some(filename.clone()),
            Arc::new(RusqliteDriver::new()),
        ));
        conn.open().await.unwrap();
        sessions.insert(
            name.clone(),
            Session {
                command: attach(SqliteCommand::new(conn.clone())),
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
            fresh = attach(SqliteCommand::new(Arc::new(SqliteConnection::new(
                None,
                Arc::new(RusqliteDriver::new()),
            ))));
            &fresh
        } else {
            &sessions[op.connection.as_deref().unwrap_or(&names[0])].command
        };
        let args = params(&op.parameters);
        let expected_error = op.expect.as_ref().and_then(|e| e.error.as_deref());
        if step.execute.is_some() {
            match target.execute(&sql, Some(&args)).await {
                Ok(response) => {
                    assert!(
                        expected_error.is_none(),
                        "{}: expected {expected_error:?}",
                        spec.name
                    );
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
            match target.execute_reader(&sql, Some(&args)).await {
                Ok(reader) => {
                    assert!(
                        expected_error.is_none(),
                        "{}: expected {expected_error:?}",
                        spec.name
                    );
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
    assert_logs(&spec, dialect, &logs.lock().unwrap());
}

fn signature(line: &str) -> String {
    let body = line.split_once("]-").map(|(_, rest)| rest).unwrap_or(line);
    if let Some(idx) = body.rfind("] ") {
        let suffix = &body[idx + 2..];
        if suffix.ends_with("ms") || suffix.ends_with("μs") {
            return body[..=idx].to_string();
        }
    }
    body.to_string()
}

fn expected_signatures(spec: &Spec, dialect: &DialectFile) -> Vec<String> {
    spec.steps
        .iter()
        .filter_map(|step| {
            step.execute
                .as_ref()
                .or(step.execute_reader.as_ref())
                .map(|op| resolve_sql(&op.sql, dialect).trim().to_string())
        })
        .flat_map(|query| {
            [
                format!("[datasource][Start]-[{query}]"),
                format!("[datasource][Finish]-[{query}]"),
            ]
        })
        .collect()
}

fn assert_logs(spec: &Spec, dialect: &DialectFile, logs: &[String]) {
    let expected = expected_signatures(spec, dialect);
    let actual: Vec<String> = logs.iter().map(|line| signature(line)).collect();
    let mismatch = expected
        .iter()
        .zip(actual.iter())
        .position(|(want, got)| want != got)
        .or_else(|| (expected.len() != actual.len()).then_some(expected.len().min(actual.len())));
    if let Some(index) = mismatch {
        panic!(
            "{}: first logging mismatch at {index}\nexpected: {}\nactual:   {}\n--- expected ---\n{}\n--- actual ---\n{}",
            spec.name,
            expected.get(index).map(String::as_str).unwrap_or("<missing>"),
            actual.get(index).map(String::as_str).unwrap_or("<missing>"),
            expected.join("\n"),
            actual.join("\n"),
        );
    }
}

#[tokio::test]
async fn shared_specs() {
    let dialect = load_dialect("sqlite");
    for entry in fs::read_dir(specs_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            run(&path, &dialect).await;
        }
    }
}
