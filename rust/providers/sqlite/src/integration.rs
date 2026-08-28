use super::*;
use datasources::{Command, Connection, IsolationLevel, Parameter, Reader, Transaction, Value};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
struct Spec {
    name: String,
    filename: Option<String>,
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
    rows: Option<Vec<BTreeMap<String, serde_yaml::Value>>>,
    error: Option<String>,
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

fn specs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("specs/sqlite")
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
    }
}

async fn collect(reader: &dyn Reader) -> Vec<datasources::Record> {
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

async fn run(path: &std::path::Path) {
    let spec: Spec = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let names = spec
        .connections
        .clone()
        .filter(|items| !items.is_empty())
        .unwrap_or_else(|| vec!["default".into()]);
    let filename = shared(
        &spec.filename.clone().unwrap_or_else(|| ":memory:".into()),
        names.len(),
    );
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
                command: SqliteCommand::new(conn.clone()),
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
            let isolation = start
                .isolation
                .as_deref()
                .map(IsolationLevel::from_name);
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
        let fresh;
        let target = if op.open == Some(false) {
            fresh = SqliteCommand::new(Arc::new(SqliteConnection::new(
                None,
                Arc::new(RusqliteDriver::new()),
            )));
            &fresh
        } else {
            &sessions[op.connection.as_deref().unwrap_or(&names[0])].command
        };
        let args = params(&op.parameters);
        let expected_error = op.expect.as_ref().and_then(|e| e.error.as_deref());
        if step.execute.is_some() {
            match target.execute(Some(&op.sql), Some(&args)).await {
                Ok(response) => {
                    assert!(expected_error.is_none(), "{}: expected {expected_error:?}", spec.name);
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
            match target.execute_reader(Some(&op.sql), Some(&args)).await {
                Ok(reader) => {
                    assert!(expected_error.is_none(), "{}: expected {expected_error:?}", spec.name);
                    if let Some(expected) = op.expect.as_ref().and_then(|e| e.rows.as_ref()) {
                        let actual = collect(reader.as_ref()).await;
                        assert_eq!(actual.len(), expected.len(), "{}", spec.name);
                        for (row, want) in actual.iter().zip(expected) {
                            for (key, value) in want {
                                assert_eq!(norm(row.get(key).unwrap()), *value, "{key}");
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
}

#[tokio::test]
async fn shared_specs() {
    for entry in fs::read_dir(specs_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            run(&path).await;
        }
    }
}
