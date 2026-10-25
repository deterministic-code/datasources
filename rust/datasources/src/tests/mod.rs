
fn stmt_text(statement: &dyn crate::Statement) -> String {
    crate::DefaultStatementTextRenderer::instance().render(statement)
}
fn where_sql(clause: &dyn crate::IWhere) -> String {
    crate::DefaultStatementTextRenderer::instance().render_where(clause)
}

#[allow(dead_code)]
mod bulk_write_strategy;
mod repository_builder_options_factory;
mod command;
mod connection;
mod contact;
mod datasource;
mod datasource_factory;
mod datasource_url_parser;
mod statement_compile;
mod nested_select_builder;
mod statement_mappers;
mod statement_parser;
mod transaction;
mod unit_of_work_transaction;

use crate::converters::{Cells, RowTypeConverter};
use futures_util::TryStreamExt;
use crate::*;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

pub(crate) fn env_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|error| error.into_inner())
}

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

#[test]
fn isolation_from_name() {
    assert_eq!(
        IsolationLevel::from_name("readUncommitted"),
        IsolationLevel::ReadUncommitted
    );
    assert_eq!(
        IsolationLevel::from_name("readCommitted"),
        IsolationLevel::ReadCommitted
    );
    assert_eq!(
        IsolationLevel::from_name("repeatableRead"),
        IsolationLevel::RepeatableRead
    );
    assert_eq!(
        IsolationLevel::from_name("serializable"),
        IsolationLevel::Serializable
    );
    assert_eq!(
        IsolationLevel::from_name("other"),
        IsolationLevel::Serializable
    );
}

#[test]
fn errors_display() {
    let installed = DatasourceError::not_installed("mysql");
    let implemented = DatasourceError::not_implemented("oracle");
    let message = DatasourceError::message("broken");
    assert_eq!(installed.to_string(), "Provider \"mysql\" is not installed");
    assert_eq!(
        implemented.to_string(),
        "Provider \"oracle\" is not implemented"
    );
    assert_eq!(message.to_string(), "broken");
    assert_eq!(format!("{installed:?}").contains("mysql"), true);
    let _ = installed.clone();
    assert_eq!(installed, DatasourceError::not_installed("mysql"));
}

#[test]
fn values_and_parameters() {
    assert_eq!(Value::from(true), Value::Bool(true));
    assert_eq!(Value::from(3_i64), Value::Int(3));
    assert_eq!(Value::from(3_i32), Value::Int(3));
    assert_eq!(Value::from(1.5_f64), Value::Float(1.5));
    assert_eq!(Value::from("x"), Value::Text("x".into()));
    assert_eq!(Value::from(String::from("y")), Value::Text("y".into()));
    let p = Parameter::new("id", 1_i64);
    assert_eq!(p.name, "id");
    assert_eq!(parameters_to_map::<Value>(None).len(), 0);
    assert_eq!(parameters_to_map(Some(&[p])).len(), 1);
    let response = Response::new(Value::Null, 2, None);
    assert_eq!(response.rows_affected, 2);
}

#[tokio::test]
async fn readers() {
    let empty = ArrayReader::<Row>::new(vec![]);
    assert!(empty.current().is_none());
    assert!(!empty.next().await.unwrap());
    let reader = ArrayReader::new(vec![record(&[("id", Value::Int(1))])]);
    assert!(reader.next().await.unwrap());
    assert!(reader.current().is_some());
    assert!(!reader.next().await.unwrap());
    assert!(!reader.next_result().await.unwrap());

    struct Sets {
        sets: Vec<Vec<Row>>,
        set: Mutex<usize>,
        row: Mutex<isize>,
        closed: Arc<Mutex<bool>>,
    }
    impl Drop for Sets {
        fn drop(&mut self) {
            *self.closed.lock().expect("reader") = true;
        }
    }
    #[async_trait]
    impl Reader for Sets {
        fn current(&self) -> Option<Row> {
            let set = *self.set.lock().expect("reader");
            let row = *self.row.lock().expect("reader");
            if row < 0 {
                return None;
            }
            self.sets.get(set).and_then(|rows| rows.get(row as usize).cloned())
        }
        async fn next(&self) -> Result<bool> {
            let set = *self.set.lock().expect("reader");
            let mut row = self.row.lock().expect("reader");
            *row += 1;
            Ok(self.sets.get(set).is_some_and(|rows| (*row as usize) < rows.len()))
        }
        async fn next_result(&self) -> Result<bool> {
            let mut set = self.set.lock().expect("reader");
            *set += 1;
            *self.row.lock().expect("reader") = -1;
            Ok(*set < self.sets.len())
        }
    }
    let closed = Arc::new(Mutex::new(false));
    let live = Sets {
        sets: vec![
            vec![record(&[("id", Value::Int(1))]), record(&[("id", Value::Int(2))])],
            vec![record(&[("id", Value::Int(3))])],
        ],
        set: Mutex::new(0),
        row: Mutex::new(-1),
        closed: closed.clone(),
    };
    let (reader, rows) = FirstResultReader::capture(Box::new(live)).await.unwrap();
    assert_eq!(rows.len(), 2);
    assert!(reader.next().await.unwrap());
    assert_eq!(reader.current().unwrap().get("id"), Some(&Value::Int(1)));
    assert!(reader.next().await.unwrap());
    assert_eq!(reader.current().unwrap().get("id"), Some(&Value::Int(2)));
    assert!(!reader.next().await.unwrap());
    assert!(reader.next_result().await.unwrap());
    assert!(reader.next().await.unwrap());
    assert_eq!(reader.current().unwrap().get("id"), Some(&Value::Int(3)));
    drop(reader);
    assert!(*closed.lock().expect("reader"));

    let collected: Vec<_> = ArrayReader::new(vec![record(&[("id", Value::Int(2))])])
        .read()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(collected.len(), 1);

    struct SkipReader;
    #[async_trait]
    impl Reader for SkipReader {
        fn current(&self) -> Option<Row> {
            None
        }
        async fn next(&self) -> Result<bool> {
            Ok(true)
        }
    }
    struct OnceSkip {
        called: Mutex<bool>,
    }
    #[async_trait]
    impl Reader for OnceSkip {
        fn current(&self) -> Option<Row> {
            None
        }
        async fn next(&self) -> Result<bool> {
            let mut called = self.called.lock().unwrap();
            if *called {
                return Ok(false);
            }
            *called = true;
            Ok(true)
        }
    }
    let _ = SkipReader;
    let items: Vec<_> = OnceSkip {
        called: Mutex::new(false),
    }
    .read()
    .try_collect()
    .await
    .unwrap();
    assert!(items.is_empty());

    let typed = ArrayReader::new(vec!["x".to_string(), "y".to_string()]);
    assert!(typed.next().await.unwrap());
    assert_eq!(typed.current().as_deref(), Some("x"));
    assert!(typed.next().await.unwrap());
    assert!(!typed.next().await.unwrap());
    let collected: Vec<_> = ArrayReader::new(vec!["z".to_string()])
        .read()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(collected, vec!["z".to_string()]);
}

struct RecordingMiddleware {
    before: Arc<dyn Fn(&str) -> Result<Option<(String, Option<Vec<Parameter>>)>> + Send + Sync>,
    after: Arc<dyn Fn(&str, Option<&DatasourceError>) -> Result<()> + Send + Sync>,
}

#[async_trait]
impl CommandMiddleware for RecordingMiddleware {
    async fn before_command(
        &self,
        text: &str,
        _params: Option<&[Parameter]>,
    ) -> Result<Option<(String, Option<Vec<Parameter>>)>> {
        (self.before)(text)
    }

    async fn after_command(
        &self,
        text: &str,
        _params: Option<&[Parameter]>,
        _results: &[Row],
        _elapsed_ms: f64,
        error: Option<&DatasourceError>,
    ) -> Result<()> {
        (self.after)(text, error)
    }
}

#[tokio::test]
async fn middleware_paths() {
    let host = CommandMiddlewareHost::new();
    assert_eq!(
        host.run("", None, |_q, _p| async { Ok((1, vec![])) })
            .await
            .unwrap(),
        1
    );

    let order = Arc::new(Mutex::new(Vec::new()));
    let seen = order.clone();
    host.add_middleware(Arc::new(RecordingMiddleware {
        before: {
            let seen = seen.clone();
            Arc::new(move |query| {
                seen.lock().unwrap().push(format!("b:{query}"));
                Ok(Some(("SELECT 2".into(), None)))
            })
        },
        after: {
            let seen = seen.clone();
            Arc::new(move |query, error| {
                seen.lock().unwrap().push(format!("a:{query}"));
                assert!(error.is_none());
                Ok(())
            })
        },
    }));
    let used = Arc::new(Mutex::new(String::new()));
    let used_clone = used.clone();
    host.run("SELECT 1", None, move |query, _p| {
        let used_clone = used_clone.clone();
        async move {
            *used_clone.lock().unwrap() = query;
            Ok(((), vec![Row::new()]))
        }
    })
    .await
    .unwrap();
    assert_eq!(*used.lock().unwrap(), "SELECT 2");
    assert_eq!(
        *order.lock().unwrap(),
        vec!["b:SELECT 1".to_string(), "a:SELECT 2".to_string()]
    );

    let blocked = CommandMiddlewareHost::new();
    let ran = Arc::new(Mutex::new(false));
    blocked.add_middleware(Arc::new(RecordingMiddleware {
        before: Arc::new(|_| Err(DatasourceError::message("blocked"))),
        after: {
            let ran = ran.clone();
            Arc::new(move |_, _| {
                *ran.lock().unwrap() = true;
                Ok(())
            })
        },
    }));
    assert!(blocked
        .run("select", None, {
            let ran = ran.clone();
            move |_q, _p| {
                let ran = ran.clone();
                async move {
                    *ran.lock().unwrap() = true;
                    Ok((1, vec![]))
                }
            }
        })
        .await
        .is_err());
    assert!(!*ran.lock().unwrap());

    let failed = CommandMiddlewareHost::new();
    let captured = Arc::new(Mutex::new(None));
    failed.add_middleware(Arc::new(RecordingMiddleware {
        before: Arc::new(|_| Ok(None)),
        after: {
            let captured = captured.clone();
            Arc::new(move |_, error| {
                *captured.lock().unwrap() = error.map(ToString::to_string);
                Ok(())
            })
        },
    }));
    assert!(failed
        .run("select", None, |_q, _p| async {
            Err::<((), Vec<Row>), _>(DatasourceError::message("fail-op"))
        })
        .await
        .is_err());
    assert_eq!(*captured.lock().unwrap(), Some("fail-op".into()));
}

#[test]
fn console_format() {
    assert_eq!(format_elapsed(2.0), "2ms");
    assert_eq!(format_elapsed(1.0), "1ms");
    assert_eq!(format_elapsed(0.85), "850μs");
    assert_eq!(format_elapsed(0.4), "400μs");
    assert_eq!(
        utc_iso_millis(std::time::UNIX_EPOCH + std::time::Duration::from_millis(0)),
        "1970-01-01T00:00:00.000Z"
    );
    assert_eq!(
        utc_iso_millis(std::time::UNIX_EPOCH + std::time::Duration::from_secs(200 * 86_400)),
        "1970-07-20T00:00:00.000Z"
    );
    assert_eq!(
        utc_iso_millis(std::time::UNIX_EPOCH - std::time::Duration::from_secs(1)),
        "1970-01-01T00:00:00.000Z"
    );
    let _ = ConsoleCommandMiddleware::new();
    let _ = ConsoleCommandMiddleware::default();
}

#[tokio::test]
async fn console_lines() {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let stamps = Arc::new(Mutex::new(vec![
        "2026-08-28T01:54:12.101Z".to_string(),
        "2026-08-28T01:54:12.103Z".to_string(),
    ]));
    let write_lines = lines.clone();
    let write_stamps = stamps.clone();
    let middleware = ConsoleCommandMiddleware::with_sink(
        move |line| write_lines.lock().unwrap().push(line.to_string()),
        move || write_stamps.lock().unwrap().remove(0),
    );
    let host = CommandMiddlewareHost::new();
    host.add_middleware(Arc::new(middleware));
    host.run(
        r#"SELECT * FROM "widget" ORDER BY "id""#,
        None,
        |_q, _p| async { Ok(((), vec![])) },
    )
    .await
    .unwrap();
    let logged = lines.lock().unwrap().clone();
    assert_eq!(
        logged[0],
        r#"[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT * FROM "widget" ORDER BY "id"]"#
    );
    assert!(logged[1].starts_with(
        r#"[2026-08-28T01:54:12.103Z]-[datasource][Finish]-[SELECT * FROM "widget" ORDER BY "id"] "#
    ));
}

#[tokio::test]
async fn console_trims_query() {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let write_lines = lines.clone();
    let middleware = ConsoleCommandMiddleware::with_sink(
        move |line| write_lines.lock().unwrap().push(line.to_string()),
        || "2026-08-28T01:54:12.101Z".to_string(),
    );
    let host = CommandMiddlewareHost::new();
    host.add_middleware(Arc::new(middleware));
    host.run("SELECT 1\n", None, |_q, _p| async { Ok(((), vec![])) })
        .await
        .unwrap();
    let logged = lines.lock().unwrap()[0].clone();
    let signature = logged.replacen("[2026-08-28T01:54:12.101Z]-", "", 1);
    assert_eq!(signature, "[datasource][Start]-[SELECT 1]");
}

#[tokio::test]
async fn console_verbose_bodies() {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let stamps = Arc::new(Mutex::new(vec![
        "2026-08-28T01:54:12.101Z".to_string(),
        "2026-08-28T01:54:12.102Z".to_string(),
        "2026-08-28T01:54:12.103Z".to_string(),
        "2026-08-28T01:54:12.104Z".to_string(),
    ]));
    let write_lines = lines.clone();
    let write_stamps = stamps.clone();
    let middleware = ConsoleCommandMiddleware::with_sink(
        move |line| write_lines.lock().unwrap().push(line.to_string()),
        move || write_stamps.lock().unwrap().remove(0),
    )
    .options(HashMap::from([("isVerbose".into(), "true".into())]));
    let host = CommandMiddlewareHost::new();
    host.add_middleware(Arc::new(middleware));
    host.run(
        "SELECT 1",
        Some(&[Parameter::new("id", 1i64)]),
        |_q, _p| async { Ok(((), vec![record(&[("id", Value::Int(1))])])) },
    )
    .await
    .unwrap();
    let logged = lines.lock().unwrap().clone();
    assert_eq!(logged.len(), 4);
    assert_eq!(
        logged[0],
        "[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]"
    );
    assert_eq!(
        logged[1],
        "[2026-08-28T01:54:12.102Z]-[datasource][Request]-[{id: 1}]"
    );
    assert!(logged[2].starts_with("[2026-08-28T01:54:12.103Z]-[datasource][Finish]-[SELECT 1] "));
    assert_eq!(
        logged[3],
        "[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{id: 1}]"
    );
    lines.lock().unwrap().clear();
    stamps.lock().unwrap().extend([
        "2026-08-28T01:54:12.101Z".to_string(),
        "2026-08-28T01:54:12.102Z".to_string(),
        "2026-08-28T01:54:12.103Z".to_string(),
        "2026-08-28T01:54:12.104Z".to_string(),
    ]);
    host.run(
        "DELETE FROM user",
        Some(&[Parameter::new("id", 2i64)]),
        |_q, _p| async { Ok((Response::new(Value::Null, 1, None), vec![])) },
    )
    .await
    .unwrap();
    let deleted = lines.lock().unwrap()[3].clone();
    assert!(deleted.contains("[AfterCommand]-[{"));
    assert!(deleted.contains("rowsAffected: 1"));
    assert!(deleted.contains("result: null"));
    lines.lock().unwrap().clear();
    stamps.lock().unwrap().extend([
        "2026-08-28T01:54:12.101Z".to_string(),
        "2026-08-28T01:54:12.102Z".to_string(),
        "2026-08-28T01:54:12.103Z".to_string(),
        "2026-08-28T01:54:12.104Z".to_string(),
    ]);
    host.run(
        "SELECT 1",
        Some(&[
            Parameter::new("name", "Ada"),
            Parameter::new("missing", Value::Null),
            Parameter::new("on", true),
            Parameter::new("off", false),
        ]),
        |_q, _p| async {
            Ok((
                (),
                vec![record(&[
                    ("name", Value::Text("Ada".into())),
                    ("missing", Value::Null),
                    ("on", Value::Bool(true)),
                    ("off", Value::Bool(false)),
                ])],
            ))
        },
    )
    .await
    .unwrap();
    let logged = lines.lock().unwrap().clone();
    assert_eq!(
        logged[1],
        "[2026-08-28T01:54:12.102Z]-[datasource][Request]-[{name: Ada, missing: null, on: True, off: False}]"
    );
    assert_eq!(
        logged[3],
        "[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{missing: null, name: Ada, off: False, on: True}]"
    );
}

#[tokio::test]
async fn expect_console_matches_transcript() {
    let now = || "2026-08-28T01:54:12.101Z".to_string();
    let middleware = Arc::new(ExpectConsoleMiddleware::new(
        vec![
            "[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]".into(),
            "[2026-08-28T01:54:12.101Z]-[datasource][Request]-[{id: 1}]".into(),
            "[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]".into(),
            "[2026-08-28T01:54:12.101Z]-[datasource][AfterCommand]-[{id: 1}]".into(),
        ],
        now,
    ));
    let host = CommandMiddlewareHost::new();
    host.add_middleware(middleware.clone());
    host.run(
        "SELECT 1",
        Some(&[Parameter::new("id", 1i64)]),
        |_q, _p| async { Ok(((), vec![record(&[("id", Value::Int(1))])])) },
    )
    .await
    .unwrap();
    middleware.complete().unwrap();
}

#[tokio::test]
async fn expect_console_applies_a_custom_line_check() {
    let now = || "2026-08-28T01:54:12.101Z".to_string();
    let middleware = Arc::new(ExpectConsoleMiddleware::with_normalize(
        vec![
            "[<date>]-[datasource][Start]-[SELECT 1]".into(),
            "[<date>]-[datasource][Request]-[{id: 1}]".into(),
            "[<date>]-[datasource][Finish]-[SELECT 1]".into(),
            "[<date>]-[datasource][AfterCommand]-[{id: 1}]".into(),
        ],
        now,
        |line| {
            crate::expect_console_middleware::signature(line)
                .replace("2026-08-28T01:54:12.101Z", "<date>")
        },
    ));
    let host = CommandMiddlewareHost::new();
    host.add_middleware(middleware.clone());
    host.run(
        "SELECT 1",
        Some(&[Parameter::new("id", 1i64)]),
        |_q, _p| async { Ok(((), vec![record(&[("id", Value::Int(1))])])) },
    )
    .await
    .unwrap();
    middleware.complete().unwrap();
}

#[tokio::test]
async fn expect_console_rejects_a_different_line() {
    let middleware = Arc::new(ExpectConsoleMiddleware::new(vec!["[nope]".into()], || {
        "2026-08-28T01:54:12.101Z".to_string()
    }));
    let host = CommandMiddlewareHost::new();
    host.add_middleware(middleware.clone());
    let error = host
        .run(
            "SELECT 1",
            Some(&[Parameter::new("id", 1i64)]),
            |_q, _p| async { Ok(((), vec![record(&[("id", Value::Int(1))])])) },
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("console mismatch at 0"));
    let short = ExpectConsoleMiddleware::new(vec!["[missing]".into()], || {
        "2026-08-28T01:54:12.101Z".to_string()
    });
    assert!(short
        .complete()
        .unwrap_err()
        .to_string()
        .contains("actual:   <missing>"));
}

#[test]
fn parses_dotenv() {
    let parsed = crate::dot_env::parse(
        "# hi\n\nPROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n=skip\n",
    );
    assert_eq!(parsed.get("PROVIDER").map(String::as_str), Some("sqlite"));
    assert_eq!(
        parsed.get("DATASOURCE_URL").map(String::as_str),
        Some("sqlite:///:memory:")
    );
    assert!(crate::dot_env::load("missing.env").is_empty());
}

#[test]
fn registry() {
    let _guard = env_lock();
    reset();
    for key in [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ] {
        std::env::remove_var(key);
    }
    assert!(registered().is_empty());
    register("json", MockDriver::arc());
    assert_eq!(registered(), vec!["json".to_string()]);
    let created = create(Some(&HashMap::from([("PROVIDER".into(), "json".into())]))).unwrap();
    assert_eq!(created.provider(), "json");
    assert!(default().is_err());
    std::env::set_var("DATASOURCE_URL", "json://");
    assert!(default().is_err());
    std::env::remove_var("DATASOURCE_URL");
    std::env::set_var("PROVIDER", "json");
    assert_eq!(default().unwrap().provider(), "json");
    reset();
    assert!(create(Some(&HashMap::from([("PROVIDER".into(), "json".into())]))).is_err());
    assert!(default().is_err());
}

#[tokio::test]
async fn stubs_throw() {
    let ds = create_stub_datasource("text");
    assert_eq!(Datasource::provider(&*ds), "text");
    let closed = ds.create_connection().await.unwrap();
    assert!(!closed.is_open());
    assert!(closed.open().await.is_err());
    assert!(ds.create_connection().await.unwrap().close().await.is_err());
    assert!(ds
        .create_connection()
        .await
        .unwrap()
        .start_transaction(None)
        .await
        .is_err());
    assert!(ds.create_command(None).await.is_err());
    let command = closed.create_command();
    assert!(command
        .execute_native(&Native::new("x"), None)
        .await
        .is_err());
    assert!(command
        .execute_reader_native(&Native::new("select"), Some(&[Parameter::new("id", 1_i64)]))
        .await
        .is_err());
    let ds: &dyn Datasource = &*ds;
    let repo = ds.create_repository::<Cells>(None, None);
    let mut empty = Row::new();
    assert!(repo.read_single(None, None, None).await.is_err());
    assert!(repo.read(None, None, None).try_collect::<Vec<_>>().await.is_err());
    assert!(repo
        .read_single(Some(&WhereText::new("1=1")), None, None)
        .await
        .is_err());
    assert!(repo.read(Some(&WhereText::new("1=1")), None, None).try_collect::<Vec<_>>().await.is_err());
    assert!(repo.create(&mut empty).await.is_err());
    assert!(repo.update(&mut empty, None).await.is_err());
    assert!(repo.delete(&empty, None).await.is_err());
    let readonly = ds.create_readonly_repository::<Cells>(None, None);
    assert!(readonly.read_single(None, None, None).await.is_err());
    assert!(readonly.read(None, None, None).try_collect::<Vec<_>>().await.is_err());
    assert!(readonly
        .read_single(Some(&WhereText::new("1=1")), None, None)
        .await
        .is_err());
    assert!(readonly
        .read(Some(&WhereText::new("1=1")), None, None)
        .try_collect::<Vec<_>>().await
        .is_err());
    let conn = StubConnection::new("text");
    let txn = StubTransaction::new("text", Arc::new(StubConnection::new("text")));
    assert!(txn.commit().await.is_err());
    assert!(txn.abort().await.is_err());
    let reader = StubReader::<Row>::new("text");
    assert!(reader.current().is_none());
    assert!(reader.next().await.is_err());
    let cmd = StubCommand::new("text", Arc::new(StubConnection::new("text")));
    cmd.add_middleware(Arc::new(RecordingMiddleware {
        before: Arc::new(|_| Ok(None)),
        after: Arc::new(|_, _| Ok(())),
    }));
    let _ = conn;
    let _ = txn;
}

#[tokio::test]
async fn published_mocks() {
    let connection = MockConnection::new();
    connection.open().await.unwrap();
    assert!(connection.is_open());
    assert_eq!(connection.open_calls(), 1);
    connection
        .start_transaction(Some(IsolationLevel::ReadUncommitted))
        .await
        .unwrap();
    assert_eq!(connection.last_isolation(), IsolationLevel::ReadUncommitted);
    connection.close().await.unwrap();
    connection.set_open_error("open-fail");
    assert!(connection.open().await.is_err());
    connection.set_close_error("close-fail");
    assert!(connection.close().await.is_err());
    connection.set_start_error("tx-fail");
    assert!(connection.start_transaction(None).await.is_err());
    let custom = Arc::new(MockTransaction::new_unbound());
    let with_txn = MockConnection::with_transaction(custom);
    with_txn.start_transaction(None).await.unwrap();

    let txn = MockTransaction::new_unbound();
    txn.set_isolation(IsolationLevel::ReadCommitted);
    assert_eq!(txn.isolation(), IsolationLevel::ReadCommitted);
    txn.commit().await.unwrap();
    assert!(txn.committed());
    txn.abort().await.unwrap();
    assert!(txn.aborted());
    txn.set_commit_error("c");
    assert!(txn.commit().await.is_err());
    txn.set_abort_error("a");
    assert!(txn.abort().await.is_err());

    let command = MockCommand::new();
    command.execute_native(&crate::Native::new("insert"), None).await.unwrap();
    command
        .execute_reader_native(
            &crate::Native::new(""),
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    command.set_execute_error("exec");
    assert!(command
        .execute_native(&crate::Native::new("x"), None)
        .await
        .is_err());
    command.set_reader_error("read");
    assert!(command
        .execute_reader_native(&crate::Native::new("x"), None)
        .await
        .is_err());

    let reader = MockReader::new(vec![record(&[("id", Value::Int(1))])]);
    assert!(reader.next().await.unwrap());
    reader.set_error("next");
    assert!(reader.next().await.is_err());

    let repo = MockRepository::new();
    assert!(repo.read_single(None, None, None).await.unwrap().is_none());
    repo.items
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1))]));
    assert!(repo
        .read_single(Some(&WhereText::new("id=1")), None, None)
        .await
        .unwrap()
        .is_some());
    assert_eq!(
        repo.read(Some(&MockSelectStatement::new("all")), None, None)
            .try_collect::<Vec<_>>().await
            .unwrap()
            .len(),
        1
    );
    assert!(repo
        .read_single(Some(&MockWhere::new("id=1")), None, None)
        .await
        .unwrap()
        .is_some());
    assert_eq!(
        repo.read(Some(&MockWhere::new("all")), None, None)
            .try_collect::<Vec<_>>().await
            .unwrap()
            .len(),
        1
    );
    let created = repo
        .create(&mut record(&[("id", Value::Int(2))]))
        .await
        .unwrap();
    assert_eq!(created["id"], Value::Int(2));
    let mut updated_row_1 = record(&[("id", Value::Int(2))]);
    let updated = repo
        .update(
            &mut updated_row_1,
            Some(&MockWhere::new("id=2")),
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    let removed = repo
        .delete(&record(&[("id", Value::Int(2))]), None)
        .await
        .unwrap();
    assert_eq!(removed.rows_affected, 1);
    repo.set_read_single_error("gs");
    assert!(repo.read_single(None, None, None).await.is_err());
    repo.set_read_error("g");
    assert!(repo.read(None, None, None).try_collect::<Vec<_>>().await.is_err());
    repo.set_add_error("ad");
    assert!(repo.create(&mut Row::new()).await.is_err());
    repo.set_update_error("up");
    let mut updated_row_2 = Row::new();
    assert!(repo.update(&mut updated_row_2, None).await.is_err());
    repo.set_delete_error("del");
    assert!(repo.delete(&Row::new(), None).await.is_err());

    let ds = MockDatasource::new("mock");
    let ds: &dyn Datasource = &ds;
    assert_eq!(Datasource::provider(ds), "mock");
    ds.create_connection().await.unwrap();
    let command = ds.create_command(None).await.unwrap();
    command
        .execute_native(&crate::Native::new(""), None)
        .await
        .unwrap();
    command
        .execute_reader_native(&crate::Native::new(""), None)
        .await
        .unwrap();
    ds.create_repository::<Cells>(None, None);
    ds.create_readonly_repository::<Cells>(None, None);

    let q = MockSelectStatement::new("q");
    let w = MockWhere::new("w");
    assert_eq!(stmt_text(&q), "q");
    assert_eq!(where_sql(&w), "w");
    assert_eq!(q.accept_calls(), 1);
    assert_eq!(w.accept_calls(), 1);
    assert_eq!(where_sql(&WhereText::new("x")), "x");

    let command = MockCommand::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    command.add_middleware(Arc::new(RecordingMiddleware {
        before: {
            let seen = seen.clone();
            Arc::new(move |query| {
                seen.lock().unwrap().push(format!("b:{query}"));
                Ok(None)
            })
        },
        after: {
            let seen = seen.clone();
            Arc::new(move |query, _| {
                seen.lock().unwrap().push(format!("a:{query}"));
                Ok(())
            })
        },
    }));
    command.execute_native(&crate::Native::new("insert"), None).await.unwrap();
    assert_eq!(
        *seen.lock().unwrap(),
        vec!["b:insert".to_string(), "a:insert".to_string()]
    );
}

#[test]
fn statements_and_builders() {
    let where_clause = Where::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    assert_eq!(where_sql(&where_clause), "id = $id AND age > $age");
    assert_eq!(where_clause.parameters().len(), 2);
    assert_eq!(
        Where::new(Operator::not_equals("id", 2_i64))
            .parameters()
            .len(),
        1
    );
    assert_eq!(
        Where::new(Operator::less_than("n", 0_i64))
            .parameters()
            .len(),
        1
    );
    assert_eq!(
        Where::new(Operator::greater_than_or_equal("n", 0_i64))
            .parameters()
            .len(),
        1
    );
    assert_eq!(
        Where::new(Operator::less_than_or_equal("n", 0_i64))
            .parameters()
            .len(),
        1
    );
    assert!(Where::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(1_i64))),
        right: Box::new(Expression::Literal(Literal::new(1_i64))),
    })
    .parameters()
    .is_empty());
    assert_eq!(where_clause.parameters()[0].value, Value::Int(1));
    assert_eq!(where_clause.parameters()[1].value, Value::Int(18));
    assert_eq!(
        where_sql(&Where::new(Operator::not_equals("id", 2_i64))),
        "id <> $id"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::less_than("n", 0_i64))),
        "n < $n"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::greater_than_or_equal("n", 0_i64))),
        "n >= $n"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::less_than_or_equal("n", 0_i64))),
        "n <= $n"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(1_i64))),
        })),
        "1 = 1"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(0_i64))),
        })),
        "1 = 0"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::equals("id", 1_i64))),
        "id = $id"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::or(
            where_clause.clone(),
            Operator::equals("name", "a")
        ))),
        "(id = $id AND age > $age) OR name = $name"
    );
    assert_eq!(
        Where::new(Operator::or(
            where_clause.clone(),
            Operator::equals("name", "a")
        ))
        .parameters()
        .len(),
        3
    );
    let not_where = Where::new(Operator::not(Where::new(Operator::equals("id", 1_i64))));
    assert_eq!(where_sql(&not_where), "NOT (id = $id)");
    assert_eq!(not_where.parameters()[0].name, "id");
    assert_eq!(
        where_sql(&Where::new(Operator::and(
            Operator::or(Operator::equals("id", 1_i64), Operator::equals("id", 2_i64)),
            Operator::equals("age", 18_i64)
        ))),
        "(id = $id OR id = $id) AND age = $age"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::or(
            Operator::in_list("id", vec![Value::Int(1)]),
            Operator::equals("name", "a")
        ))),
        "(id IN ($id_0)) OR name = $name"
    );
    assert_eq!(
        where_sql(&Where::new(Expression::Identifier(Identifier::new("id")))),
        "id"
    );
    assert!(Where::new(Expression::Identifier(Identifier::new("id")))
        .parameters()
        .is_empty());
    assert_eq!(
        where_sql(&Where::new(Expression::Array(ArrayExpression::new(vec![
            Expression::Literal(Literal::new(1_i64))
        ])))),
        "1"
    );
    assert!(Where::new(Expression::Array(ArrayExpression::new(vec![
        Expression::Literal(Literal::new(1_i64))
    ])))
    .parameters()
    .is_empty());
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Identifier(Identifier::new("a"))),
            right: Box::new(Expression::Identifier(Identifier::new("b"))),
        })),
        "a = b"
    );
    assert!(Where::new(Operator::Equals {
        left: Box::new(Expression::Identifier(Identifier::new("a"))),
        right: Box::new(Expression::Identifier(Identifier::new("b"))),
    })
    .parameters()
    .is_empty());
    assert_eq!(
        where_sql(&Where::new(Operator::In {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(2_i64))),
        })),
        "1 IN 2"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(true))),
            right: Box::new(Expression::Literal(Literal::new(false))),
        })),
        "true = false"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(Value::Null))),
            right: Box::new(Expression::Literal(Literal::new(Value::Null))),
        })),
        "null = null"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1.5_f64))),
            right: Box::new(Expression::Literal(Literal::new(2.5_f64))),
        })),
        "1.5 = 2.5"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                rust_decimal::Decimal::new(15, 1)
            )))),
            right: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                rust_decimal::Decimal::new(25, 1)
            )))),
        })),
        "1.5 = 2.5"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new("a"))),
            right: Box::new(Expression::Literal(Literal::new("b"))),
        })),
        "'a' = 'b'"
    );
    assert!(where_sql(&Where::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::Uuid(
            uuid::Uuid::nil()
        )))),
        right: Box::new(Expression::Literal(Literal::new(Value::Uuid(
            uuid::Uuid::nil()
        )))),
    }))
    .contains('='));
    assert!(where_sql(&Where::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::DateTime(
            chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
        )))),
        right: Box::new(Expression::Literal(Literal::new(Value::DateTime(
            chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
        )))),
    }))
    .contains('='));
    assert!(where_sql(&Where::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
        right: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
    }))
    .contains('='));
    assert!(Where::new(Operator::In {
        left: Box::new(Expression::Literal(Literal::new(1_i64))),
        right: Box::new(Expression::Literal(Literal::new(2_i64))),
    })
    .parameters()
    .is_empty());
    assert_eq!(
        where_sql(&Where::new(Operator::and(
            Operator::not(Operator::equals("id", 1_i64)),
            Operator::equals("age", 18_i64)
        ))),
        "(NOT (id = $id)) AND age = $age"
    );
    assert_eq!(
        where_sql(&Where::new(Operator::in_list("id", vec![]))),
        "1 = 0"
    );
    assert!(Where::new(Operator::in_list("id", vec![]))
        .parameters()
        .is_empty());
    let in_ids = Where::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)]));
    assert_eq!(where_sql(&in_ids), "id IN ($id_0, $id_1)");
    assert_eq!(in_ids.parameters()[0].name, "id_0");
    assert_eq!(in_ids.parameters()[1].name, "id_1");
    match &where_clause.condition {
        Expression::Operator(Operator::And { left, .. }) => match left.as_ref() {
            Expression::Operator(Operator::Equals { left, right }) => {
                match left.as_ref() {
                    Expression::Identifier(id) => assert_eq!(id.name, "id"),
                    _ => panic!("id"),
                }
                match right.as_ref() {
                    Expression::Literal(lit) => assert_eq!(lit.value, Value::Int(1)),
                    _ => panic!("lit"),
                }
            }
            _ => panic!("left"),
        },
        _ => panic!("and"),
    }

    let fields = [Parameter::new("id", 1_i64), Parameter::new("name", "a")];
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new("users", &["id", "name"])
    });
    assert_eq!(
        stmt_text(&query),
        "SELECT id, name FROM users WHERE id = $id AND age > $age"
    );
    assert!(query.where_clause().is_some());
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions::new("users", &[]))),
        "SELECT * FROM users"
    );
    assert!(
        SelectStatement::new(SelectStatementOptions::new("users", &[]))
            .where_clause()
            .is_none()
    );
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions::new("users", &["id", "name"]))),
        "SELECT id, name FROM users"
    );
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions {
            joins: &[Join::inner("orders", "users.id", "orders.user_id")],
            ..SelectStatementOptions::new("users", &["users.id", "orders.total"])
        })),
        "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id"
    );
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions {
            where_clause: Some(&Where::new(Operator::equals("users.id", 1_i64))),
            joins: &[
                Join::inner("orders", "users.id", "orders.user_id"),
                Join::left("payments", "orders.id", "payments.order_id"),
                Join::right("profiles", "users.id", "profiles.user_id"),
            ],
            ..SelectStatementOptions::new("users", &["users.id"])
        })),
        "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id"
    );
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions {
            joins: &[],
            ..SelectStatementOptions::new("users", &["id"])
        })),
        "SELECT id FROM users"
    );
    let count = Aggregate::count("*");
    let total = Aggregate::sum("amount");
    let grouped_fields = ["status", count.as_str(), total.as_str()];
    let grouped = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&Where::new(Operator::equals("status", "open"))),
        joins: &[Join::inner(
            "customers",
            "orders.customer_id",
            "customers.id",
        )],
        group_by: &["status"],
        order_by: &[OrderBy::desc("total"), OrderBy::asc("status")],
        limit: Some(10),
        aliases: &[None, Some("n"), Some("total")],
        ..SelectStatementOptions::new("orders", &grouped_fields)
    });
    assert_eq!(
        stmt_text(&grouped),
        "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10"
    );
    let by_status = SelectStatement::new(SelectStatementOptions {
        group_by: &["id"],
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(stmt_text(&by_status), "SELECT id FROM users GROUP BY id");
    let min_n = Aggregate::min("n");
    let max_n = Aggregate::max("n");
    let bound_fields = [min_n.as_str(), max_n.as_str()];
    let bounds = SelectStatement::new(SelectStatementOptions::new("users", &bound_fields));
    assert_eq!(stmt_text(&bounds), "SELECT MIN(n), MAX(n) FROM users");
    let ordered = SelectStatement::new(SelectStatementOptions {
        order_by: &[OrderBy::asc("id"), OrderBy::desc("name")],
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(
        stmt_text(&ordered),
        "SELECT id FROM users ORDER BY id ASC, name DESC"
    );
    let limited = SelectStatement::new(SelectStatementOptions {
        limit: Some(10),
        ..SelectStatementOptions::new("users", &["id"])
    });
    assert_eq!(stmt_text(&limited), "SELECT id FROM users LIMIT 10");
    assert_eq!(
        stmt_text(&SelectStatement::new(SelectStatementOptions {
            limit: Some(0),
            ..SelectStatementOptions::new("users", &["id"])
        })),
        "SELECT id FROM users LIMIT 0"
    );
    let insert = InsertStatement::new("users", &fields);
    assert_eq!(
        stmt_text(&insert),
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
    let update = UpdateStatement::new("users", &fields, Some(&where_clause));
    assert_eq!(
        stmt_text(&update),
        "UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age"
    );
    assert!(update.where_clause().is_some());
    assert!(UpdateStatement::new("users", &fields, None)
        .where_clause()
        .is_none());
    let delete = DeleteStatement::new("users", Some(&where_clause));
    assert_eq!(
        stmt_text(&delete),
        "DELETE FROM users WHERE id = $id AND age > $age"
    );
    assert!(delete.where_clause().is_some());
    assert_eq!(
        stmt_text(&DeleteStatement::new("users", None)),
        "DELETE FROM users"
    );
    let create_table = CreateTableStatement::new(
        "users",
        &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")],
        &[],
    );
    assert_eq!(
        stmt_text(&create_table),
        "CREATE TABLE users (id INTEGER, name TEXT)"
    );
    struct ReplacedRenderer;
    impl CreateTableStatementTextRenderer for ReplacedRenderer {
        fn render(&self, _statement: &CreateTableStatement) -> String {
            "CREATE TABLE replaced".into()
        }
    }
    assert_eq!(
        ReplacedRenderer.render(&create_table),
        "CREATE TABLE replaced"
    );
    let named_table = CreateTableStatement::new(
        "orders",
        &[
            Column::new("id", "INTEGER")
                .default(ColumnDefault::new("0", Some("df_id")))
                .not_null(NotNull::new(Some("nn_id"))),
            Column::new("tenant_id", "INTEGER"),
            Column::new("user_id", "INTEGER"),
            Column::new("qty", "INTEGER"),
        ],
        &[
            PrimaryKey::new("pk_orders", &["id", "tenant_id"]),
            ForeignKey::new(
                "fk_orders_user",
                &["user_id"],
                "users",
                &["id"],
                Some(ReferentialAction::Cascade),
                None,
            ),
            UniqueConstraint::new("uq_orders_user", &["user_id"]),
            CheckConstraint::new("chk_orders_qty", "qty > 0"),
        ],
    );
    assert_eq!(
        stmt_text(&named_table),
        "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))"
    );
    assert_eq!(
        stmt_text(&CreateTableStatement::new(
            "items",
            &[
                Column::new("id", "INTEGER").default(ColumnDefault::new("0", None)),
                Column::new("name", "TEXT").not_null(NotNull::new(None)),
            ],
            &[ForeignKey::new("fk_items_user", &["id"], "users", &["id"], None, None)],
        )),
        "CREATE TABLE items (id INTEGER DEFAULT 0, name TEXT NOT NULL, CONSTRAINT fk_items_user FOREIGN KEY (id) REFERENCES users (id))"
    );
    assert_eq!(
        stmt_text(&CreateTableStatement::new(
            "t",
            &[Column::new("id", "INTEGER")],
            &[ForeignKey::new(
                "fk_set_null",
                &["id"],
                "u",
                &["id"],
                Some(ReferentialAction::SetNull),
                Some(ReferentialAction::SetDefault),
            )],
        )),
        "CREATE TABLE t (id INTEGER, CONSTRAINT fk_set_null FOREIGN KEY (id) REFERENCES u (id) ON DELETE SET NULL ON UPDATE SET DEFAULT)"
    );
    assert_eq!(
        stmt_text(&CreateTableStatement::new(
            "t",
            &[Column::new("id", "INTEGER")],
            &[ForeignKey::new(
                "fk_restrict",
                &["id"],
                "u",
                &["id"],
                Some(ReferentialAction::Restrict),
                Some(ReferentialAction::NoAction),
            )],
        )),
        "CREATE TABLE t (id INTEGER, CONSTRAINT fk_restrict FOREIGN KEY (id) REFERENCES u (id) ON DELETE RESTRICT ON UPDATE NO ACTION)"
    );
    let create_index = CreateIndexStatement::new("idx_orders_user", "orders", &["user_id"], false);
    assert_eq!(
        stmt_text(&create_index),
        "CREATE INDEX idx_orders_user ON orders (user_id)"
    );
    assert_eq!(
        stmt_text(&CreateIndexStatement::new("uq_orders_code", "orders", &["code"], true)),
        "CREATE UNIQUE INDEX uq_orders_code ON orders (code)"
    );
    let call = ExecuteStoredProcedureStatement::new("save", &fields);
    assert_eq!(stmt_text(&call), "CALL save($id, $name)");
    assert_eq!(
        stmt_text(&ExecuteStoredProcedureStatement::new("save", &[])),
        "CALL save()"
    );
    let ret = ReturnStatement::new("add", &fields);
    assert_eq!(stmt_text(&ret), "SELECT add($id, $name)");
    assert_eq!(stmt_text(&ReturnStatement::new("add", &[])), "SELECT add()");
    assert_eq!(
        stmt_text(&ExecuteStoredProcedureStatement::new(
            "save",
            &[Parameter::new("id", 1_i64), Parameter::return_value("ret")]
        )),
        "CALL save($id)"
    );
    assert_eq!(
        stmt_text(&ReturnStatement::new(
            "add",
            &[Parameter::new("id", 1_i64), Parameter::return_value("ret")]
        )),
        "SELECT add($id)"
    );
    assert_eq!(
        Parameter::new("id", 1_i64).direction,
        ParameterDirection::Input
    );
    assert_eq!(
        Parameter::output("count").direction,
        ParameterDirection::Output
    );
    assert_eq!(Parameter::output("count").value, Value::Null);
    assert!(Response::new(1_i64, 2, None).parameters.is_none());
    assert_eq!(
        stmt_text(&InsertStatement::new("t", &[])),
        "INSERT INTO t () VALUES ()"
    );
    assert_eq!(
        stmt_text(&UpdateStatement::new(
            "users",
            &[Parameter::new("name", "a")],
            None
        )),
        "UPDATE users SET name = $name"
    );
    assert_eq!(
        stmt_text(&DeleteStatement::new("users", None)),
        "DELETE FROM users"
    );
    assert_eq!(
        stmt_text(&CreateTableStatement::new("t", &[], &[])),
        "CREATE TABLE t ()"
    );
    assert_eq!(
        stmt_text(&CreateIndexStatement::new("idx", "t", &["id"], false)),
        "CREATE INDEX idx ON t (id)"
    );
    assert_eq!(
        stmt_text(&ExecuteStoredProcedureStatement::new("save", &[])),
        "CALL save()"
    );
    assert_eq!(
        stmt_text(&ReturnStatement::new("add", &[])),
        "SELECT add()"
    );
    let mock = MockSelectStatement::with_where("q", MockWhere::new("w"));
    assert!(mock.where_clause().is_some());

}

#[test]
#[should_panic(expected = "negative limit")]
fn rejects_a_negative_limit() {
    let _ = stmt_text(&SelectStatement::new(SelectStatementOptions {
        limit: Some(-1),
        ..SelectStatementOptions::new("users", &["id"])
    }));
}

#[test]
fn evaluates_where_statement() {
    let evaluator = ExpressionEvaluator;
    let row = record(&[
        ("id", Value::Int(1)),
        ("age", Value::Int(21)),
        ("name", Value::Text("a".into())),
    ]);
    let yes = Value::Bool(true);
    let no = Value::Bool(false);
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::equals("id", 1_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::equals("id", 2_i64)), &row),
        no
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::not_equals("id", 2_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::greater_than("age", 18_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::less_than("age", 30_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::greater_than_or_equal("age", 21_i64)),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::less_than_or_equal("age", 21_i64)),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::and(
                Operator::equals("id", 1_i64),
                Operator::greater_than("age", 18_i64)
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::and(
                Operator::equals("id", 0_i64),
                Operator::greater_than("age", 18_i64)
            )),
            &row
        ),
        no
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::or(
                Operator::equals("id", 0_i64),
                Operator::equals("name", "a")
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::or(
                Operator::equals("id", 1_i64),
                Operator::equals("name", "z")
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::not(Operator::equals("id", 1_i64))),
            &row
        ),
        no
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)])),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(0_i64))),
            }),
            &row
        ),
        no
    );
    assert_eq!(
        evaluator.evaluate(&Expression::Identifier(Identifier::new("missing")), &row),
        Value::Null
    );
    assert_eq!(
        evaluator.evaluate(&Expression::Literal(Literal::new(Value::Null)), &row),
        Value::Null
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::less_than("name", "b")), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &Where::new(Operator::greater_than_or_equal("name", "a")),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&Where::new(Operator::greater_than("name", "Z")), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate(
            &Expression::Operator(Operator::GreaterThan {
                left: Box::new(Expression::Literal(Literal::new(2.5_f64))),
                right: Box::new(Expression::Literal(Literal::new(1.5_f64))),
            }),
            &row
        ),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(
            &Expression::Operator(Operator::GreaterThan {
                left: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                    rust_decimal::Decimal::new(25, 1)
                )))),
                right: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                    rust_decimal::Decimal::new(15, 1)
                )))),
            }),
            &row
        ),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(
            &Expression::Operator(Operator::GreaterThan {
                left: Box::new(Expression::Literal(Literal::new(2_i64))),
                right: Box::new(Expression::Literal(Literal::new(1.5_f64))),
            }),
            &row
        ),
        Value::Bool(true)
    );
    assert_eq!(
        evaluator.evaluate(
            &Expression::Operator(Operator::GreaterThan {
                left: Box::new(Expression::Literal(Literal::new(2.5_f64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }),
            &row
        ),
        Value::Bool(true)
    );
}

#[test]
#[should_panic(expected = "array is not a scalar")]
fn evaluate_array_panics() {
    ExpressionEvaluator.evaluate(
        &Expression::Array(ArrayExpression::new(vec![])),
        &Row::new(),
    );
}

#[test]
#[should_panic(expected = "text where is not evaluable")]
fn evaluate_text_where_panics() {
    ExpressionEvaluator.evaluate_where(&WhereText::new("id = 1"), &Row::new());
}

#[test]
#[should_panic(expected = "empty in")]
fn evaluate_empty_in_panics() {
    ExpressionEvaluator.evaluate(
        &Expression::Operator(Operator::In {
            left: Box::new(Expression::Identifier(Identifier::new("id"))),
            right: Box::new(Expression::Literal(Literal::new(1_i64))),
        }),
        &Row::new(),
    );
}

#[test]
#[should_panic(expected = "empty compare")]
fn evaluate_empty_compare_panics() {
    ExpressionEvaluator.evaluate(
        &Expression::Operator(Operator::GreaterThan {
            left: Box::new(Expression::Literal(Literal::new(true))),
            right: Box::new(Expression::Literal(Literal::new(1_i64))),
        }),
        &Row::new(),
    );
}

#[test]
#[should_panic(expected = "empty in")]
fn collect_in_non_literal_panics() {
    let _ = Where::new(Operator::In {
        left: Box::new(Expression::Identifier(Identifier::new("id"))),
        right: Box::new(Expression::Array(ArrayExpression::new(vec![
            Expression::Identifier(Identifier::new("x")),
        ]))),
    })
    .parameters();
}

#[tokio::test]
async fn executes_statement_helpers() {
    let command = MockCommand::new();
    let fields = [Parameter::new("id", 1_i64)];
    let statement = ExecuteStoredProcedureStatement::new("save", &fields);
    command
        .execute_statement(&statement, Some(&fields))
        .await
        .unwrap();
    assert_eq!(command.execute_logs.lock().unwrap()[0].0, "CALL save($id)");
    command
        .execute_reader_statement(&statement, Some(&fields))
        .await
        .unwrap();
    assert_eq!(command.reader_logs.lock().unwrap()[0].0, "CALL save($id)");
}

fn users_mapping() -> RowTypeConverter {
    RowTypeConverter::of::<Item>()
}

#[tokio::test]
async fn read_calls_stored_procedure() {
    let command = Arc::new(MockCommand::new());
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1)), ("name", Value::from("a"))]));
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(users_mapping());
    let repo = CRUDRepository::new(connection, options).unwrap();
    let parameters = [Parameter::new("id", 9_i64)];
    let call = ExecuteStoredProcedureStatement::new("get_user", &parameters);
    let rows = repo.read(Some(&call), Some(&parameters), None).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["id"], Value::Int(1));
    assert_eq!(rows[0]["name"], Value::from("a"));
    let log = command.reader_logs.lock().unwrap()[0].clone();
    assert_eq!(log.0, "CALL get_user($id)");
    assert_eq!(log.1[0].name, "id");
    assert_eq!(log.1[0].value, Value::Int(9));
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1)), ("name", Value::from("a"))]));
    assert_eq!(
        repo.read_single(Some(&call), Some(&parameters), None)
            .await
            .unwrap()
            .unwrap()["id"],
        Value::Int(1)
    );
}

#[tokio::test]
async fn table_repository() {
    let command = Arc::new(MockCommand::new());
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1)), ("name", Value::from("a"))]));
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(users_mapping());
    let repo = Arc::new(CRUDRepository::new(connection, options).unwrap());
    repo.add_middleware(Arc::new(RecordingMiddleware {
        before: Arc::new(|_| Ok(None)),
        after: Arc::new(|_, _| Ok(())),
    }));
    let rows = repo.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT id, name FROM users"
    );
    assert!(repo.read_single(None, None, None).await.unwrap().is_some());

    let where_clause = Where::new(Operator::equals("id", 9_i64));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new("users", &["id"])
    });
    command.reader_rows.lock().unwrap().clear();
    assert!(repo
        .read_single(Some(&query), None, None)
        .await
        .unwrap()
        .is_none());
    assert_eq!(
        command.reader_logs.lock().unwrap().last().unwrap().0,
        "SELECT id FROM users WHERE id = $id"
    );

    command.reader_rows.lock().unwrap().clear();
    assert!(repo
        .read_single(Some(&Where::new(Operator::equals("id", 9_i64))), None, None)
        .await
        .unwrap()
        .is_none());
    let last = command.reader_logs.lock().unwrap().last().unwrap().clone();
    assert_eq!(last.0, "SELECT id, name FROM users WHERE id = $id");
    assert_eq!(last.1[0].name, "id");
    assert_eq!(last.1[0].value, Value::Int(9));
    assert!(repo
        .read(Some(&Where::new(Operator::equals("id", 9_i64))), None, None)
        .try_collect::<Vec<_>>().await
        .unwrap()
        .is_empty());

    let mut obj = record(&[("id", Value::Int(0)), ("name", Value::from("a"))]);
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut include_id = CRUDRepositoryOptions::default();
    include_id.entity = "users".into();
    include_id.row_converter = Some(users_mapping());
    let repo = CRUDRepository::new(connection, include_id).unwrap();
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(9))]));
    let created = repo.create(&mut obj, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (name) VALUES ($name)"
    );
    assert_eq!(created["id"], Value::Int(9));
    let updated = repo.update(&mut obj, None, None).await.unwrap();
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE users SET name = $name WHERE id = $id"
    );
    let removed = repo.delete(&obj, None, None).await.unwrap();
    assert_eq!(removed.rows_affected, 1);
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "DELETE FROM users WHERE id = $id"
    );
}

#[tokio::test]
async fn create_and_update_honor_write_properties() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["Id".into()];
    options.entity = "addresses".into();
    options.read_properties = vec!["Id".into(), "Line1".into(), "Description".into()];
    options.create_properties = Some(vec!["Line1".into()]);
    options.update_properties = Some(vec!["Line1".into()]);
    let repo = CRUDRepository::new(connection, options).unwrap();
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("Id", Value::Int(1))]));
    repo.create(&mut record(&[
        ("Id", Value::Int(0)),
        ("Line1", Value::from("1 Main")),
        ("Description", Value::from("Home")),
    ]), None)
    .await
    .unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO addresses (Line1) VALUES ($Line1)"
    );
    repo.update(
        &mut record(&[
            ("Id", Value::Int(1)),
            ("Line1", Value::from("2 Main")),
            ("Description", Value::from("Work")),
        ]),
        None,
        None,
    )
    .await
    .unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE addresses SET Line1 = $Line1 WHERE Id = $Id"
    );
}

#[tokio::test]
async fn create_and_update_write_read_properties_only() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["Id".into()];
    options.entity = "addresses".into();
    options.read_properties = vec!["Id".into(), "Line1".into()];
    let repo = CRUDRepository::new(connection, options).unwrap();
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("Id", Value::Int(1))]));
    repo.create(&mut record(&[
        ("Id", Value::Int(0)),
        ("Line1", Value::from("1 Main")),
        ("Description", Value::from("Home")),
    ]), None)
    .await
    .unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO addresses (Line1) VALUES ($Line1)"
    );
    repo.update(
        &mut record(&[
            ("Id", Value::Int(1)),
            ("Line1", Value::from("2 Main")),
            ("Description", Value::from("Work")),
        ]),
        None,
        None,
    )
    .await
    .unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE addresses SET Line1 = $Line1 WHERE Id = $Id"
    );
}

#[tokio::test]
async fn table_repository_assigned_id() {
    let assigned = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(assigned.clone(), "memory");
    let mut assigned_options = CRUDRepositoryOptions::default();
    assigned_options.entity = "users".into();
    assigned_options.row_converter = Some(users_mapping());
    let repo = CRUDRepository::new(connection, assigned_options).unwrap();
    *assigned.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    let created = repo
        .create(&mut record(&[
            ("id", Value::Int(42)),
            ("name", Value::from("ada")),
        ]), None)
        .await
        .unwrap();
    assert_eq!(
        assigned.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
    assert!(assigned.reader_logs.lock().unwrap().is_empty());
    assert_eq!(created["id"], Value::Int(42));

    let generated = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(generated.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["org_id".into(), "user_id".into()];
    options.entity = "users".into();
    options.row_converter = Some(RowTypeConverter::of::<Composite>());
    let repo = CRUDRepository::new(connection, options).unwrap();
    *generated.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    let created = repo
        .create(&mut record(&[
            ("org_id", Value::Int(1)),
            ("user_id", Value::Int(2)),
            ("name", Value::Int(3)),
        ]), None)
        .await
        .unwrap();
    assert_eq!(created["org_id"], Value::Int(1));
    assert_eq!(created["user_id"], Value::Int(2));
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (name, org_id, user_id) VALUES ($name, $org_id, $user_id)"
    );
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].1[0].value,
        Value::Int(3)
    );
    let mut updated_row_3 = record(&[
                ("org_id", Value::Int(1)),
                ("user_id", Value::Int(2)),
                ("name", Value::Int(4)),
            ]);
    let updated = repo
        .update(
            &mut updated_row_3,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    assert!(generated.execute_logs.lock().unwrap()[1]
        .0
        .contains("org_id = $org_id AND user_id = $user_id"));

    let command = Arc::new(MockCommand::new());
    CRUDRepository::new(
        MockConnection::with_command(command.clone()),
        CRUDRepositoryOptions::default(),
    )
    .unwrap();
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.row_converter = Some(users_mapping());
    let repo = CRUDRepository::new(connection, options).unwrap();
    let mut updated_row_4 = record(&[("name", Value::from("a"))]);
    assert!(repo
        .update(&mut updated_row_4, None, None)
        .await
        .is_err());
    assert!(repo
        .delete(&record(&[("name", Value::from("a"))]), None, None)
        .await
        .is_err());
    let mut updated_row_5 = record(&[("id", Value::Int(1))]);
    assert!(repo
        .update(&mut updated_row_5, None, None)
        .await
        .is_err());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(RowTypeConverter::of::<Cells>());
    let repo = CRUDRepository::new(connection, options).unwrap();
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    let created = repo
        .create(&mut record(&[("id", Value::Int(1))]), None)
        .await
        .unwrap();
    assert_eq!(created["id"], Value::Int(1));
    let removed = repo
        .delete(
            &record(&[("name", Value::from("z"))]),
            Some(&Where::new(Operator::equals("name", "z"))),
            None,
        )
        .await
        .unwrap();
    assert_eq!(removed.rows_affected, 1);

    let from_id = create_crud_repository_options::<Item>();
    assert_eq!(from_id.ids, vec!["id".to_string()]);
    assert!(create_crud_repository_options::<Label>().ids.is_empty());
    CRUDRepository::new(MockConnection::new(), from_id).unwrap();
    CRUDRepository::from_type::<Item>(MockConnection::new()).unwrap();
}

#[test]
fn repository_entity_type() {
    let typed = CRUDRepository::from_type::<Item>(MockConnection::new()).unwrap();
    assert_eq!(typed.entity_type(), TypeId::of::<Item>());
    assert_eq!(
        (&typed as &dyn IRepository).entity_type(),
        TypeId::of::<Item>()
    );
    let untyped =
        CRUDRepository::new(MockConnection::new(), create_crud_repository_options::<Item>()).unwrap();
    assert_eq!(untyped.entity_type(), TypeId::of::<Row>());
    assert_eq!(
        StubRepository::new("memory").entity_type(),
        TypeId::of::<Row>()
    );
    let memory = CRUDRepository::memory(vec![]);
    assert_eq!(memory.entity_type(), TypeId::of::<Row>());
}

#[tokio::test]
async fn table_repository_crud_mapper() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(users_mapping());
    options.entity_mapper = Some(std::sync::Arc::new(TransformMapper::new(
        |name: String| match name.as_str() {
            "users" => "people".into(),
            _ => name,
        },
        |name| name,
    )));
    options.property_mappings = Some(std::sync::Arc::new(TransformMapper::new(
        |name: String| match name.as_str() {
            "id" => "user_id".into(),
            "name" => "full_name".into(),
            _ => name,
        },
        |name| name,
    )));
    let repo = CRUDRepository::new(connection, options).unwrap();
    command.reader_rows.lock().unwrap().clear();
    assert!(repo.read_single(None, None, None).await.unwrap().is_none());
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT full_name, user_id FROM people"
    );
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    let mut row = record(&[("id", Value::Int(0)), ("name", Value::from("a"))]);
    let created = repo.create(&mut row, None).await.unwrap();
    assert_eq!(created["id"], Value::Int(0));
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO people (full_name) VALUES ($full_name)"
    );
    let updated = repo.update(&mut row, None, None).await.unwrap();
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE people SET full_name = $full_name WHERE user_id = $user_id"
    );
    let removed = repo.delete(&row, None, None).await.unwrap();
    assert_eq!(removed.rows_affected, 1);
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "DELETE FROM people WHERE user_id = $user_id"
    );
}

#[tokio::test]
async fn read_uses_per_call_object_and_row_converters() {
    struct FixedLabel(&'static str);
    impl TypeConverter<Row, Row> for FixedLabel {
        fn convert(&self, _from: &Row) -> Result<Row> {
            Ok(record(&[("Label", Value::from(self.0))]))
        }
        fn reverse_convert(&self, to: &Row) -> Result<Row> {
            Ok(to.clone())
        }
    }

    struct TagLabel;
    impl TypeConverter<Row, Row> for TagLabel {
        fn convert(&self, from: &Row) -> Result<Row> {
            let label = match from.get("Label") {
                Some(Value::Text(text)) => format!("tag:{text}"),
                _ => "tag:".into(),
            };
            Ok(record(&[("Label", Value::from(label))]))
        }
        fn reverse_convert(&self, to: &Row) -> Result<Row> {
            Ok(to.clone())
        }
    }

    struct UpperLabel;
    impl TypeConverter<Value, Value> for UpperLabel {
        fn convert(&self, from: &Value) -> Result<Value> {
            Ok(match from {
                Value::Text(text) => Value::from(text.to_uppercase()),
                other => other.clone(),
            })
        }
        fn reverse_convert(&self, to: &Value) -> Result<Value> {
            Ok(to.clone())
        }
    }

    let command = Arc::new(MockCommand::new());
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("Label", Value::from("ada"))]));
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = "items".into();
    options.row_converter = Some(RowTypeConverter::new(vec!["Label".into()]));
    options.object_converter = Some(Box::new(FixedLabel("default")));
    let repo = ReadonlyRepository::new(connection, options).unwrap();
    assert_eq!(
        repo.read_single(None, None, None).await.unwrap().unwrap()["Label"],
        Value::from("default")
    );

    command.reader_rows.lock().unwrap().clear();
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("Label", Value::from("ada"))]));
    let mut per_call = ReadonlyRepositoryMappingOptions::default();
    per_call.object_converter = Some(Box::new(TagLabel));
    per_call.row_converter = Some(RowTypeConverter::new(vec!["Label".into()]).with_fields(
        HashMap::from([(
            "Label".into(),
            Box::new(UpperLabel) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        )]),
    ));
    assert_eq!(
        repo.read_single(None, None, Some(&per_call))
            .await
            .unwrap()
            .unwrap()["Label"],
        Value::from("tag:ADA")
    );
}

struct Item;

impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
    }
}

struct Stamp;

impl ConvertSchema for Stamp {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("name", "TEXT"),
            ("created", "DATETIME"),
            ("version", "BLOB"),
        ]
    }
}

struct Composite;

impl ConvertSchema for Composite {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("org_id", "INTEGER"),
            ("user_id", "INTEGER"),
            ("name", "TEXT"),
        ]
    }
}

#[tokio::test]
async fn read_single_by_id_uses_id_columns() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(users_mapping());
    let repo = CRUDRepository::new(connection, options).unwrap();
    assert!(repo
        .read_single_by_id(&Value::Int(9))
        .await
        .unwrap()
        .is_none());
    let log = command.reader_logs.lock().unwrap()[0].clone();
    assert_eq!(log.0, "SELECT id, name FROM users WHERE id = $id");
    assert_eq!(log.1[0].name, "id");
    assert_eq!(log.1[0].value, Value::Int(9));

    let generated = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(generated.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["org_id".into(), "user_id".into()];
    options.entity = "users".into();
    options.row_converter = Some(RowTypeConverter::of::<Composite>());
    let repo = CRUDRepository::new(connection, options).unwrap();
    assert!(repo
        .read_single_by_id(&record(&[
            ("org_id", Value::Int(1)),
            ("user_id", Value::Int(2)),
        ]))
        .await
        .unwrap()
        .is_none());
    assert!(generated.reader_logs.lock().unwrap()[0]
        .0
        .contains("org_id = $org_id AND user_id = $user_id"));
    assert!(repo
        .read_single_by_id(&Value::Int(1))
        .await
        .unwrap_err()
        .to_string()
        .contains("Composite id requires a row keyed by each Id column"));
    assert!(repo
        .read_single_by_id(&record(&[("org_id", Value::Int(1))]))
        .await
        .unwrap_err()
        .to_string()
        .contains("Missing Id value"));
}

struct Label;

impl ConvertSchema for Label {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT")]
    }
}

#[test]
fn comparison_aliases() {
    let equals = Where::binary("username", eq, "grace");
    let not_equals = Where::binary("id", ne, 2_i64);
    let greater = Where::binary("n", gt, 0);
    let less = Where::binary("n", lt, 0);
    let greater_or_equal = Where::binary("n", ge, 0);
    let less_or_equal = Where::binary("n", le, 0);
    let both = equals.clone().and("is_enabled", eq, 0);
    let either = equals.clone().or("is_enabled", eq, 0);
    assert_eq!(where_sql(&equals), "username = $username");
    assert_eq!(where_sql(&not_equals), "id <> $id");
    assert_eq!(where_sql(&greater), "n > $n");
    assert_eq!(where_sql(&less), "n < $n");
    assert_eq!(where_sql(&greater_or_equal), "n >= $n");
    assert_eq!(where_sql(&less_or_equal), "n <= $n");
    assert_eq!(
        where_sql(&both),
        "username = $username AND is_enabled = $is_enabled"
    );
    assert_eq!(
        where_sql(&either),
        "username = $username OR is_enabled = $is_enabled"
    );
}

#[tokio::test]
async fn create_and_update_omit_null_datetime_and_version() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "users".into();
    options.row_converter = Some(RowTypeConverter::of::<Stamp>());
    let repo = CRUDRepository::new(connection, options).unwrap();
    let created = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
    *command.response.lock().unwrap() = Response::new(Value::Null, 1, None);
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(7))]));
    let created_ada = repo
        .create(&mut record(&[
            ("id", Value::Int(0)),
            ("name", Value::from("ada")),
            ("created", Value::Null),
            ("version", Value::Null),
        ]), None)
        .await
        .unwrap();
    assert_eq!(created_ada["id"], Value::Int(7));
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (name) VALUES ($name)"
    );
    let mut updated_row_6 = record(&[
                ("id", Value::Int(1)),
                ("name", Value::from("ada")),
                ("created", Value::Null),
                ("version", Value::Null),
            ]);
    let updated = repo
        .update(
            &mut updated_row_6,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE users SET name = $name WHERE id = $id"
    );
    let created_grace = repo
        .create(&mut record(&[
            ("id", Value::Int(0)),
            ("name", Value::from("grace")),
            ("created", Value::DateTime(created)),
            ("version", Value::Binary(vec![9, 9])),
        ]), None)
        .await
        .unwrap();
    assert_eq!(created_grace["id"], Value::Int(7));
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "INSERT INTO users (created, name, version) VALUES ($created, $name, $version)"
    );
}
