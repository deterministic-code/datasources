#[allow(dead_code)]
mod command;
mod connection;
mod contact;
mod datasource_factory;
mod transaction;
mod items_repository;
mod list_repository;

use crate::converters::{Cells, RowTypeConverter};
use crate::repository_tree_node::encode_rows;
use crate::*;
use async_trait::async_trait;
use chrono::TimeZone;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
    let response = Response::new(Value::Null, 2);
    assert_eq!(response.rows_affected, 2);
}

#[test]
fn identity_helpers() {
    let with_id = record(&[("id", Value::Int(1)), ("name", Value::from("a"))]);
    assert_eq!(identity_filter(&with_id).len(), 1);
    assert_eq!(identity_where_text(&with_id), "id = $id");
    let no_id = record(&[("name", Value::from("a")), ("city", Value::from("x"))]);
    assert_eq!(identity_filter(&no_id).len(), 2);
    assert_eq!(identity_where_text(&no_id), "city = $city AND name = $name");
    assert_eq!(
        identity_where_text(&HashMap::<String, Value>::new()),
        "1 = 1"
    );
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
    let collected = collect_reader(&ArrayReader::new(vec![record(&[("id", Value::Int(2))])]))
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
    let items = collect_reader(&OnceSkip {
        called: Mutex::new(false),
    })
    .await
    .unwrap();
    assert!(items.is_empty());

    let typed = ArrayReader::new(vec!["x".to_string(), "y".to_string()]);
    assert!(typed.next().await.unwrap());
    assert_eq!(typed.current().as_deref(), Some("x"));
    assert!(typed.next().await.unwrap());
    assert!(!typed.next().await.unwrap());
    let collected = collect_reader(&ArrayReader::new(vec!["z".to_string()]))
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
        |_q, _p| async { Ok((Response::new(Value::Null, 1), vec![])) },
    )
    .await
    .unwrap();
    let deleted = lines.lock().unwrap()[3].clone();
    assert!(deleted.contains("[AfterCommand]-[{"));
    assert!(deleted.contains("rowsAffected: 1"));
    assert!(deleted.contains("result: null"));
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
    reset();
    for key in [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "DATABASE_SCHEMA",
        "DEFAULT_PROVIDER",
        "DEFAULT_DATASOURCE_URL",
        "PROVIDER",
    ] {
        std::env::remove_var(key);
    }
    assert!(registered().is_empty());
    register(
        "json",
        |_| MockConnection::new(),
        |_| Arc::new(MockRepository::new()),
    );
    assert_eq!(registered(), vec!["json".to_string()]);
    let created = create(Some(&HashMap::from([("provider".into(), "json".into())]))).unwrap();
    assert_eq!(created.provider(), "json");
    assert!(create_default().is_err());
    std::env::set_var("DATASOURCE_URL", "json://");
    assert_eq!(create_default().unwrap().provider(), "json");
    assert_eq!(default().unwrap().provider(), "json");
    std::env::set_var("DEFAULT_PROVIDER", "json");
    std::env::set_var("DEFAULT_DATASOURCE_URL", "json://extra");
    assert_eq!(default().unwrap().provider(), "json");
    std::env::remove_var("DATASOURCE_URL");
    std::env::remove_var("DEFAULT_PROVIDER");
    std::env::remove_var("DEFAULT_DATASOURCE_URL");
    reset();
    assert!(create(Some(&HashMap::from([("provider".into(), "json".into())]))).is_err());
    assert!(create_default().is_err());
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
    let command = ds.create_command(None);
    assert!(command.execute("", None).await.is_err());
    assert!(command
        .execute_reader("select", Some(&[Parameter::new("id", 1_i64)]))
        .await
        .is_err());
    let repo = ds.create_repository(None);
    let empty = Row::new();
    assert!(repo.read_single(None).await.is_err());
    assert!(repo.read(None).await.is_err());
    assert!(repo.read_single_where(&TextWhere::new("1=1")).await.is_err());
    assert!(repo.read_where(&TextWhere::new("1=1")).await.is_err());
    assert!(repo.create(&empty).await.is_err());
    assert!(repo.update(&empty, None).await.is_err());
    assert!(repo.delete(&empty, None).await.is_err());
    let readonly = ds.create_readonly_repository(None);
    assert!(readonly.read_single(None).await.is_err());
    assert!(readonly.read(None).await.is_err());
    assert!(readonly
        .read_single_where(&TextWhere::new("1=1"))
        .await
        .is_err());
    assert!(readonly.read_where(&TextWhere::new("1=1")).await.is_err());
    let txn = StubTransaction::new("text");
    assert!(txn.commit().await.is_err());
    assert!(txn.abort().await.is_err());
    let reader = StubReader::<Row>::new("text");
    assert!(reader.current().is_none());
    assert!(reader.next().await.is_err());
    let conn = StubConnection::new("text");
    let cmd = StubCommand::new("text");
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
    command.execute("insert", None).await.unwrap();
    command
        .execute_reader("", Some(&[Parameter::new("id", 1_i64)]))
        .await
        .unwrap();
    command.set_execute_error("exec");
    assert!(command.execute("", None).await.is_err());
    command.set_reader_error("read");
    assert!(command.execute_reader("", None).await.is_err());

    let reader = MockReader::new(vec![record(&[("id", Value::Int(1))])]);
    assert!(reader.next().await.unwrap());
    reader.set_error("next");
    assert!(reader.next().await.is_err());

    let repo = MockRepository::new();
    assert!(repo.read_single(None).await.unwrap().is_none());
    repo.items
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1))]));
    assert!(repo
        .read_single(Some(&QueryStatement::text("id=1")))
        .await
        .unwrap()
        .is_some());
    assert_eq!(
        repo.read(Some(&MockQuery::new("all"))).await.unwrap().len(),
        1
    );
    assert!(repo
        .read_single_where(&MockWhere::new("id=1"))
        .await
        .unwrap()
        .is_some());
    assert_eq!(
        repo.read_where(&MockWhere::new("all")).await.unwrap().len(),
        1
    );
    repo.create(&record(&[("id", Value::Int(2))]))
        .await
        .unwrap();
    repo.update(
        &record(&[("id", Value::Int(2))]),
        Some(&MockWhere::new("id=2")),
    )
    .await
    .unwrap();
    repo.delete(&record(&[("id", Value::Int(2))]), None)
        .await
        .unwrap();
    repo.set_read_single_error("gs");
    assert!(repo.read_single(None).await.is_err());
    repo.set_read_error("g");
    assert!(repo.read(None).await.is_err());
    repo.set_add_error("ad");
    assert!(repo.create(&Row::new()).await.is_err());
    repo.set_update_error("up");
    assert!(repo.update(&Row::new(), None).await.is_err());
    repo.set_delete_error("del");
    assert!(repo.delete(&Row::new(), None).await.is_err());

    let ds = MockDatasource::new("mock");
    assert_eq!(Datasource::provider(&ds), "mock");
    ds.create_connection().await.unwrap();
    let command = ds.create_command(None);
    command.execute("", None).await.unwrap();
    command.execute_reader("", None).await.unwrap();
    ds.create_repository(None);
    ds.create_readonly_repository(None);

    let q = MockQuery::new("q");
    let w = MockWhere::new("w");
    assert_eq!(q.get_text(), "q");
    assert_eq!(w.get_text(), "w");
    assert_eq!(q.calls(), 1);
    assert_eq!(w.calls(), 1);
    assert_eq!(TextWhere::new("x").get_text(), "x");

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
    command.execute("insert", None).await.unwrap();
    assert_eq!(
        *seen.lock().unwrap(),
        vec!["b:insert".to_string(), "a:insert".to_string()]
    );
}

#[test]
fn statements_and_builders() {
    let where_clause = WhereClause::new(Operator::and(
        Operator::equals("id", 1_i64),
        Operator::greater_than("age", 18_i64),
    ));
    assert_eq!(where_clause.get_text(), "id = $id AND age > $age");
    assert_eq!(where_clause.parameters().len(), 2);
    assert_eq!(WhereClause::new(Operator::not_equals("id", 2_i64)).parameters().len(), 1);
    assert_eq!(WhereClause::new(Operator::less_than("n", 0_i64)).parameters().len(), 1);
    assert_eq!(WhereClause::new(Operator::greater_than_or_equal("n", 0_i64)).parameters().len(), 1);
    assert_eq!(WhereClause::new(Operator::less_than_or_equal("n", 0_i64)).parameters().len(), 1);
    assert!(WhereClause::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(1_i64))),
        right: Box::new(Expression::Literal(Literal::new(1_i64))),
    })
    .parameters()
    .is_empty());
    assert_eq!(where_clause.parameters()[0].value, Value::Int(1));
    assert_eq!(where_clause.parameters()[1].value, Value::Int(18));
    assert_eq!(WhereClause::new(Operator::not_equals("id", 2_i64)).get_text(), "id <> $id");
    assert_eq!(WhereClause::new(Operator::less_than("n", 0_i64)).get_text(), "n < $n");
    assert_eq!(WhereClause::new(Operator::greater_than_or_equal("n", 0_i64)).get_text(), "n >= $n");
    assert_eq!(WhereClause::new(Operator::less_than_or_equal("n", 0_i64)).get_text(), "n <= $n");
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(1_i64))),
        })
        .get_text(),
        "1 = 1"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(0_i64))),
        })
        .get_text(),
        "1 = 0"
    );
    assert_eq!(
        WhereClause::new(Operator::equals("id", 1_i64)).get_text(),
        "id = $id"
    );
    assert_eq!(
        WhereClause::new(Operator::or(
            where_clause.clone(),
            Operator::equals("name", "a")
        ))
        .get_text(),
        "(id = $id AND age > $age) OR name = $name"
    );
    assert_eq!(
        WhereClause::new(Operator::or(
            where_clause.clone(),
            Operator::equals("name", "a")
        ))
        .parameters()
        .len(),
        3
    );
    let not_where = WhereClause::new(Operator::not(WhereClause::new(Operator::equals("id", 1_i64))));
    assert_eq!(not_where.get_text(), "NOT (id = $id)");
    assert_eq!(not_where.parameters()[0].name, "id");
    assert_eq!(
        WhereClause::new(Operator::and(
            Operator::or(Operator::equals("id", 1_i64), Operator::equals("id", 2_i64)),
            Operator::equals("age", 18_i64)
        ))
        .get_text(),
        "(id = $id OR id = $id) AND age = $age"
    );
    assert_eq!(
        WhereClause::new(Operator::or(
            Operator::in_list("id", vec![Value::Int(1)]),
            Operator::equals("name", "a")
        ))
        .get_text(),
        "(id IN ($id_0)) OR name = $name"
    );
    assert_eq!(
        WhereClause::new(Expression::Identifier(Identifier::new("id"))).get_text(),
        "id"
    );
    assert!(WhereClause::new(Expression::Identifier(Identifier::new("id")))
        .parameters()
        .is_empty());
    assert_eq!(
        WhereClause::new(Expression::Array(ArrayExpression::new(vec![
            Expression::Literal(Literal::new(1_i64))
        ])))
        .get_text(),
        "1"
    );
    assert!(WhereClause::new(Expression::Array(ArrayExpression::new(vec![
        Expression::Literal(Literal::new(1_i64))
    ])))
    .parameters()
    .is_empty());
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Identifier(Identifier::new("a"))),
            right: Box::new(Expression::Identifier(Identifier::new("b"))),
        })
        .get_text(),
        "a = b"
    );
    assert!(WhereClause::new(Operator::Equals {
        left: Box::new(Expression::Identifier(Identifier::new("a"))),
        right: Box::new(Expression::Identifier(Identifier::new("b"))),
    })
    .parameters()
    .is_empty());
    assert_eq!(
        WhereClause::new(Operator::In {
            left: Box::new(Expression::Literal(Literal::new(1_i64))),
            right: Box::new(Expression::Literal(Literal::new(2_i64))),
        })
        .get_text(),
        "1 IN 2"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(true))),
            right: Box::new(Expression::Literal(Literal::new(false))),
        })
        .get_text(),
        "true = false"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(Value::Null))),
            right: Box::new(Expression::Literal(Literal::new(Value::Null))),
        })
        .get_text(),
        "null = null"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(1.5_f64))),
            right: Box::new(Expression::Literal(Literal::new(2.5_f64))),
        })
        .get_text(),
        "1.5 = 2.5"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                rust_decimal::Decimal::new(15, 1)
            )))),
            right: Box::new(Expression::Literal(Literal::new(Value::Decimal(
                rust_decimal::Decimal::new(25, 1)
            )))),
        })
        .get_text(),
        "1.5 = 2.5"
    );
    assert_eq!(
        WhereClause::new(Operator::Equals {
            left: Box::new(Expression::Literal(Literal::new("a"))),
            right: Box::new(Expression::Literal(Literal::new("b"))),
        })
        .get_text(),
        "a = b"
    );
    assert!(WhereClause::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::Uuid(uuid::Uuid::nil())))),
        right: Box::new(Expression::Literal(Literal::new(Value::Uuid(uuid::Uuid::nil())))),
    })
    .get_text()
    .contains('='));
    assert!(WhereClause::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::DateTime(
            chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
        )))),
        right: Box::new(Expression::Literal(Literal::new(Value::DateTime(
            chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap()
        )))),
    })
    .get_text()
    .contains('='));
    assert!(WhereClause::new(Operator::Equals {
        left: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
        right: Box::new(Expression::Literal(Literal::new(Value::Binary(vec![1])))),
    })
    .get_text()
    .contains('='));
    assert!(WhereClause::new(Operator::In {
        left: Box::new(Expression::Literal(Literal::new(1_i64))),
        right: Box::new(Expression::Literal(Literal::new(2_i64))),
    })
    .parameters()
    .is_empty());
    assert_eq!(
        WhereClause::new(Operator::and(
            Operator::not(Operator::equals("id", 1_i64)),
            Operator::equals("age", 18_i64)
        ))
        .get_text(),
        "(NOT (id = $id)) AND age = $age"
    );
    assert_eq!(WhereClause::new(Operator::in_list("id", vec![])).get_text(), "1 = 0");
    assert!(WhereClause::new(Operator::in_list("id", vec![])).parameters().is_empty());
    let in_ids = WhereClause::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)]));
    assert_eq!(in_ids.get_text(), "id IN ($id_0, $id_1)");
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
    let query = QueryStatement::new("users", &["id", "name"], Some(&where_clause), &[]);
    assert_eq!(
        query.get_text(),
        "SELECT id, name FROM users WHERE id = $id AND age > $age"
    );
    assert!(query.where_clause().is_some());
    assert_eq!(
        QueryStatement::new("users", &[], None, &[]).get_text(),
        "SELECT * FROM users"
    );
    assert!(QueryStatement::new("users", &[], None, &[])
        .where_clause()
        .is_none());
    assert_eq!(
        QueryStatement::new("users", &["id", "name"], None, &[]).get_text(),
        "SELECT id, name FROM users"
    );
    assert_eq!(
        QueryStatement::new(
            "users",
            &["users.id", "orders.total"],
            None,
            &[Join::inner("orders", "users.id", "orders.user_id")],
        )
        .get_text(),
        "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id"
    );
    assert_eq!(
        QueryStatement::new(
            "users",
            &["users.id"],
            Some(&WhereClause::new(Operator::equals("users.id", 1_i64))),
            &[
                Join::inner("orders", "users.id", "orders.user_id"),
                Join::left("payments", "orders.id", "payments.order_id"),
                Join::right("profiles", "users.id", "profiles.user_id"),
            ],
        )
        .get_text(),
        "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id"
    );
    assert_eq!(
        QueryStatement::new("users", &["id"], None, &[]).get_text(),
        "SELECT id FROM users"
    );
    assert_eq!(
        InsertStatement::new("users", &fields).get_text(),
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
    let update = UpdateStatement::new("users", &fields, Some(&where_clause));
    assert_eq!(
        update.get_text(),
        "UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age"
    );
    assert!(update.where_clause().is_some());
    assert!(UpdateStatement::new("users", &fields, None)
        .where_clause()
        .is_none());
    let delete = DeleteStatement::new("users", Some(&where_clause));
    assert_eq!(
        delete.get_text(),
        "DELETE FROM users WHERE id = $id AND age > $age"
    );
    assert!(delete.where_clause().is_some());
    assert_eq!(
        DeleteStatement::new("users", None).get_text(),
        "DELETE FROM users"
    );
    assert_eq!(
        CreateTableStatement::new(
            "users",
            &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")]
        )
        .get_text(),
        "CREATE TABLE users (id INTEGER, name TEXT)"
    );
    assert_eq!(
        StoredProcedureStatement::new("save", &fields).get_text(),
        "CALL save($id, $name)"
    );
    assert_eq!(
        StoredProcedureStatement::new("save", &[]).get_text(),
        "CALL save()"
    );

    let query = QueryStatement::with_where("SELECT 1", TextWhere::new("id = 1"));
    assert_eq!(query.get_text(), "SELECT 1");
    assert!(query.where_clause().is_some());
    assert!(QueryStatement::text("SELECT 1").where_clause().is_none());
    let mock = MockQuery::with_where("q", MockWhere::new("w"));
    assert!(mock.where_clause().is_some());
    assert_eq!(InsertStatement::text("x").get_text(), "x");
    assert_eq!(UpdateStatement::text("x", None).get_text(), "x");
    assert_eq!(DeleteStatement::raw("x", None).get_text(), "x");
    assert_eq!(CreateTableStatement::text("x").get_text(), "x");
    assert_eq!(StoredProcedureStatement::text("x").get_text(), "x");
}

#[test]
fn evaluates_where_statement() {
    let evaluator = StatementEvaluator;
    let row = record(&[("id", Value::Int(1)), ("age", Value::Int(21)), ("name", Value::Text("a".into()))]);
    let yes = Value::Bool(true);
    let no = Value::Bool(false);
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::equals("id", 1_i64)), &row), yes);
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::equals("id", 2_i64)), &row), no);
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::not_equals("id", 2_i64)), &row), yes);
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::greater_than("age", 18_i64)), &row), yes);
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::less_than("age", 30_i64)), &row), yes);
    assert_eq!(
        evaluator.evaluate_where(&WhereClause::new(Operator::greater_than_or_equal("age", 21_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&WhereClause::new(Operator::less_than_or_equal("age", 21_i64)), &row),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::and(
                Operator::equals("id", 1_i64),
                Operator::greater_than("age", 18_i64)
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::and(
                Operator::equals("id", 0_i64),
                Operator::greater_than("age", 18_i64)
            )),
            &row
        ),
        no
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::or(
                Operator::equals("id", 0_i64),
                Operator::equals("name", "a")
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::or(
                Operator::equals("id", 1_i64),
                Operator::equals("name", "z")
            )),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(&WhereClause::new(Operator::not(Operator::equals("id", 1_i64))), &row),
        no
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::in_list("id", vec![Value::Int(1), Value::Int(2)])),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }),
            &row
        ),
        yes
    );
    assert_eq!(
        evaluator.evaluate_where(
            &WhereClause::new(Operator::Equals {
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
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::less_than("name", "b")), &row), yes);
    assert_eq!(
        evaluator.evaluate_where(&WhereClause::new(Operator::greater_than_or_equal("name", "a")), &row),
        yes
    );
    assert_eq!(evaluator.evaluate_where(&WhereClause::new(Operator::greater_than("name", "Z")), &row), yes);
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
    StatementEvaluator.evaluate(&Expression::Array(ArrayExpression::new(vec![])), &Row::new());
}

#[test]
#[should_panic(expected = "text where is not evaluable")]
fn evaluate_text_where_panics() {
    StatementEvaluator.evaluate_where(&TextWhere::new("id = 1"), &Row::new());
}

#[test]
#[should_panic(expected = "empty in")]
fn evaluate_empty_in_panics() {
    StatementEvaluator.evaluate(
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
    StatementEvaluator.evaluate(
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
    let _ = WhereClause::new(Operator::In {
        left: Box::new(Expression::Identifier(Identifier::new("id"))),
        right: Box::new(Expression::Array(ArrayExpression::new(vec![
            Expression::Identifier(Identifier::new("x")),
        ]))),
    })
    .parameters();
}

#[test]
fn stored_procedure_mapper_rewrites_finished_sql() {
    let mut mapper = StatementMapper::identity("", "");
    mapper.stored_procedure = Some(Arc::new(TransformMapper::new(
        |text: String| format!("{text} /* mapped */"),
        |text| text,
    )));
    let built = StoredProcedureStatement::new("save", &[Parameter::new("id", 1_i64)]);
    let text = StatementMapper::map_sql(&mapper.stored_procedure, built.get_text());
    assert_eq!(text, "CALL save($id) /* mapped */");
}

#[tokio::test]
async fn executes_statement_helpers() {
    let command = MockCommand::new();
    let fields = [Parameter::new("id", 1_i64)];
    let statement = StoredProcedureStatement::new("save", &fields);
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
async fn table_repository() {
    let command = Arc::new(MockCommand::new());
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1)), ("name", Value::from("a"))]));
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.entity = "users".into();
    options.mapping = Some(users_mapping());
    let mut repo = Repository::new(options).unwrap();
    repo.add_middleware(Arc::new(RecordingMiddleware {
        before: Arc::new(|_| Ok(None)),
        after: Arc::new(|_, _| Ok(())),
    }));
    let rows = repo.read(None).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT id, name FROM users"
    );
    assert!(repo.read_single(None).await.unwrap().is_some());

    let where_clause = WhereClause::new(Operator::equals("id", 9_i64));
    let query = QueryStatement::new("users", &["id"], Some(&where_clause), &[]);
    command.reader_rows.lock().unwrap().clear();
    assert!(repo.read_single(Some(&query)).await.unwrap().is_none());
    assert_eq!(
        command.reader_logs.lock().unwrap().last().unwrap().0,
        "SELECT id FROM users WHERE id = $id"
    );

    command.reader_rows.lock().unwrap().clear();
    assert!(repo
        .read_single_where(&WhereClause::new(Operator::equals("id", 9_i64)))
        .await
        .unwrap()
        .is_none());
    let last = command.reader_logs.lock().unwrap().last().unwrap().clone();
    assert_eq!(last.0, "SELECT id, name FROM users WHERE id = $id");
    assert_eq!(last.1[0].name, "id");
    assert_eq!(last.1[0].value, Value::Int(9));
    assert!(repo
        .read_where(&WhereClause::new(Operator::equals("id", 9_i64)))
        .await
        .unwrap()
        .is_empty());

    let obj = record(&[("id", Value::Int(1)), ("name", Value::from("a"))]);
    let mut include_id = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    include_id.assign_identity_on_create = false;
    include_id.entity = "users".into();
    include_id.mapping = Some(users_mapping());
    let repo = Repository::new(include_id).unwrap();
    repo.create(&obj).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
    repo.update(&obj, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE users SET name = $name WHERE id = $id"
    );
    repo.delete(&obj, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "DELETE FROM users WHERE id = $id"
    );

    let generated = Arc::new(MockCommand::new());
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(generated.clone(), "memory"),
    );
    options.ids = vec!["org_id".into(), "user_id".into()];
    options.assign_identity_on_create = true;
    options.entity = "users".into();
    options.mapping = Some(RowTypeConverter::of::<Composite>());
    let repo = Repository::new(options).unwrap();
    repo.create(&record(&[
        ("org_id", Value::Int(1)),
        ("user_id", Value::Int(2)),
        ("name", Value::Int(3)),
    ]))
    .await
    .unwrap();
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (name) VALUES ($name)"
    );
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].1[0].value,
        Value::Int(3)
    );
    repo.update(
        &record(&[
            ("org_id", Value::Int(1)),
            ("user_id", Value::Int(2)),
            ("name", Value::Int(4)),
        ]),
        None,
    )
    .await
    .unwrap();
    assert!(generated.execute_logs.lock().unwrap()[1]
        .0
        .contains("org_id = $org_id AND user_id = $user_id"));

    let command = Arc::new(MockCommand::new());
    Repository::new(RepositoryBuilderOptions::with_factory(
        MockConnection::with_command(command.clone()),
    ))
    .unwrap();
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.mapping = Some(users_mapping());
    let repo = Repository::new(options).unwrap();
    assert!(repo
        .update(&record(&[("name", Value::from("a"))]), None)
        .await
        .is_err());
    assert!(repo
        .delete(&record(&[("name", Value::from("a"))]), None)
        .await
        .is_err());
    assert!(repo
        .update(&record(&[("id", Value::Int(1))]), None)
        .await
        .is_err());
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.entity = "users".into();
    options.mapping = Some(RowTypeConverter::of::<Cells>());
    let repo = Repository::new(options).unwrap();
    repo.create(&record(&[("id", Value::Int(1))]))
        .await
        .unwrap();
    repo.delete(
        &record(&[("name", Value::from("z"))]),
        Some(&WhereClause::new(Operator::equals("name", "z"))),
    )
    .await
    .unwrap();

    let from_id = create_repository_builder_options::<Item>(MockConnection::new());
    assert_eq!(from_id.ids, vec!["id".to_string()]);
    assert!(
        create_repository_builder_options::<Label>(MockConnection::new())
            .ids
            .is_empty()
    );
    Repository::new(from_id).unwrap();
    Repository::from_type::<Item>(MockConnection::new()).unwrap();
}

#[test]
fn repository_entity_type() {
    let typed = Repository::from_type::<Item>(MockConnection::new()).unwrap();
    assert_eq!(typed.entity_type(), TypeId::of::<Item>());
    assert_eq!(
        (&typed as &dyn IRepository).entity_type(),
        TypeId::of::<Item>()
    );
    let untyped = Repository::new(create_repository_builder_options::<Item>(
        MockConnection::new(),
    ))
    .unwrap();
    assert_eq!(untyped.entity_type(), TypeId::of::<Row>());
    assert_eq!(
        StubRepository::new("memory").entity_type(),
        TypeId::of::<Row>()
    );
    let deep = DeepCRUDRepository::new(Arc::new(typed));
    assert_eq!(deep.entity_type(), TypeId::of::<Item>());
    assert_eq!(
        (&deep as &dyn IRepository).entity_type(),
        TypeId::of::<Item>()
    );
}

fn user_contacts_mapper() -> TransformMapper<Row, Row> {
    TransformMapper::new(
        |mut row: Row| {
            let first = row.get("contact_name1").cloned().unwrap_or(Value::Null);
            let second = row.get("contact_name2").cloned().unwrap_or(Value::Null);
            row.insert(
                "contacts".into(),
                Value::from(format!(
                    "{},{}",
                    match first {
                        Value::Text(text) => text,
                        _ => String::new(),
                    },
                    match second {
                        Value::Text(text) => text,
                        _ => String::new(),
                    }
                )),
            );
            row
        },
        |mut row: Row| {
            if let Some(Value::Text(contacts)) = row.remove("contacts") {
                let mut parts = contacts.splitn(2, ',');
                if let Some(first) = parts.next() {
                    row.insert("contact_name1".into(), Value::from(first));
                }
                if let Some(second) = parts.next() {
                    row.insert("contact_name2".into(), Value::from(second));
                }
            }
            row
        },
    )
}

#[tokio::test]
async fn table_repository_maps_after_row_converter() {
    let command = Arc::new(MockCommand::new());
    command
        .reader_rows
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1)), ("name", Value::from("a"))]));
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.entity = "users".into();
    options.mapping = Some(users_mapping());
    options.mapper = Some(TransformMapper::new(
        |mut row: Row| {
            if let Some(Value::Text(name)) = row.get("name").cloned() {
                row.insert("name".into(), Value::from(name.to_uppercase()));
            }
            row
        },
        |mut row: Row| {
            if let Some(Value::Text(name)) = row.get("name").cloned() {
                row.insert("name".into(), Value::from(name.to_lowercase()));
            }
            row
        },
    ));
    let repo = Repository::new(options).unwrap();
    let rows = repo.read(None).await.unwrap();
    assert_eq!(rows[0]["name"], Value::from("A"));
}

#[tokio::test]
async fn table_repository_maps_flat_row_into_contacts() {
    let command = Arc::new(MockCommand::new());
    command.reader_rows.lock().unwrap().push(record(&[
        ("id", Value::Int(1)),
        ("username", Value::from("bobby")),
        ("email", Value::from("bob@asdf.com")),
        ("contact_name1", Value::from("bob")),
        ("contact_name2", Value::from("roberta")),
    ]));
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.entity = "users".into();
    options.assign_identity_on_create = false;
    options.mapping = Some(RowTypeConverter::of::<UserRow>());
    options.mapper = Some(user_contacts_mapper());
    let repo = Repository::new(options).unwrap();
    let user = repo.read_single(None).await.unwrap().unwrap();
    assert_eq!(user["username"], Value::from("bobby"));
    assert_eq!(user["email"], Value::from("bob@asdf.com"));
    assert_eq!(user["contacts"], Value::from("bob,roberta"));
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT contact_name1, contact_name2, email, id, username FROM users"
    );
    repo.create(&user).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (contact_name1, contact_name2, email, id, username) VALUES ($contact_name1, $contact_name2, $email, $id, $username)"
    );
    repo.update(&user, None).await.unwrap();
    assert!(!command.execute_logs.lock().unwrap()[1]
        .0
        .contains("contacts"));
    repo.delete(&user, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "DELETE FROM users WHERE id = $id"
    );
}

#[tokio::test]
async fn table_repository_statement_mapper() {
    let command = Arc::new(MockCommand::new());
    let names = TransformMapper::new(
        |name: String| match name.as_str() {
            "users" => "people".into(),
            "id" => "user_id".into(),
            "name" => "full_name".into(),
            _ => name,
        },
        |name| name,
    );
    let mut mapper = StatementMapper::identity("memory", "rust");
    mapper.entity = TransformMapper::new(
        |name: String| match name.as_str() {
            "users" => "people".into(),
            _ => name,
        },
        |name| name,
    );
    mapper.columns = names;
    mapper.query = Some(Arc::new(TransformMapper::new(
        |text: String| format!("{text} LIMIT 1"),
        |text| text,
    )));
    let returning = |text: String| format!("{text} RETURNING id");
    mapper.insert = Some(Arc::new(TransformMapper::new(returning, |text| text)));
    mapper.update = Some(Arc::new(TransformMapper::new(
        |text: String| format!("{text} RETURNING id"),
        |text| text,
    )));
    mapper.delete = Some(Arc::new(TransformMapper::new(
        |text: String| format!("{text} RETURNING id"),
        |text| text,
    )));
    let mut options = RepositoryBuilderOptions::with_factory(
        MockConnection::with_command_provider(command.clone(), "memory"),
    );
    options.entity = "users".into();
    options.assign_identity_on_create = false;
    options.mapping = Some(users_mapping());
    options.statement_mapper = Some(mapper);
    let repo = Repository::new(options).unwrap();
    command.reader_rows.lock().unwrap().clear();
    assert!(repo.read_single(None).await.unwrap().is_none());
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT full_name, user_id FROM people LIMIT 1"
    );
    let row = record(&[("id", Value::Int(1)), ("name", Value::from("a"))]);
    repo.create(&row).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO people (id, name) VALUES ($id, $name) RETURNING id"
    );
    repo.update(&row, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[1].0,
        "UPDATE people SET name = $name WHERE id = $id RETURNING id"
    );
    repo.delete(&row, None).await.unwrap();
    assert_eq!(
        command.execute_logs.lock().unwrap()[2].0,
        "DELETE FROM people WHERE id = $id RETURNING id"
    );
}

struct UserRow;

impl ConvertSchema for UserRow {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("username", "TEXT"),
            ("email", "TEXT"),
            ("contact_name1", "TEXT"),
            ("contact_name2", "TEXT"),
        ]
    }
}

struct Item;

impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
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

struct Label;

impl ConvertSchema for Label {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT")]
    }
}

#[tokio::test]
async fn enriched_fills_lookup_select_and_leaves_missing_default() {
    let parents = Arc::new(items_repository::ItemsRepository::new(vec![
        record(&[("id", Value::Int(1)), ("type_id", Value::Int(10))]),
        record(&[("id", Value::Int(2)), ("type_id", Value::Int(99))]),
    ]));
    let types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(10)),
        ("name", Value::from("person")),
    ])]));
    let mut repo = EnrichedReadonlyRepository::new(parents);
    repo.add_enriched(
        "type_name",
        "type_id",
        types,
        "id",
        Some("name".into()),
        false,
    );
    let items = repo.read(None).await.unwrap();
    assert_eq!(items[0]["type_name"], Value::from("person"));
    assert!(!items[1].contains_key("type_name"));
}

#[tokio::test]
async fn enriched_fills_lists_and_read_single_is_enriched() {
    let parents = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(1)),
        ("type_id", Value::Int(10)),
    ])]));
    let types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(10)),
        ("name", Value::from("org")),
    ])]));
    let children = Arc::new(items_repository::ItemsRepository::new(vec![
        record(&[("parent_id", Value::Int(1)), ("label", Value::from("a"))]),
        record(&[("parent_id", Value::Int(1)), ("label", Value::from("b"))]),
        record(&[("parent_id", Value::Int(2)), ("label", Value::from("x"))]),
    ]));
    let mut repo = EnrichedReadonlyRepository::new(parents);
    repo.add_enriched(
        "type_name",
        "type_id",
        types,
        "id",
        Some("name".into()),
        false,
    )
    .add_enriched("children", "id", children, "parent_id", None, true);
    let item = repo.read_single(None).await.unwrap().unwrap();
    assert_eq!(item["type_name"], Value::from("org"));
    assert!(item["children"].wire_text().contains("label=a"));
    assert!(item["children"].wire_text().contains("label=b"));
    assert!(!item["children"].wire_text().contains("label=x"));
}

#[tokio::test]
async fn enriched_nests_list_lookup_and_keeps_root_sibling() {
    let parents = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(1)),
        ("type_id", Value::Int(10)),
    ])]));
    let types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(10)),
        ("name", Value::from("org")),
    ])]));
    let child_types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(3)),
        ("name", Value::from("home")),
    ])]));
    let children = Arc::new(items_repository::ItemsRepository::new(vec![
        record(&[
            ("parent_id", Value::Int(1)),
            ("type_id", Value::Int(3)),
            ("label", Value::from("a")),
        ]),
        record(&[
            ("parent_id", Value::Int(1)),
            ("type_id", Value::Int(9)),
            ("label", Value::from("b")),
        ]),
    ]));
    let mut repo = EnrichedReadonlyRepository::new(parents);
    repo.add_enriched(
        "type_name",
        "type_id",
        types,
        "id",
        Some("name".into()),
        false,
    )
    .add_enriched_with(
        "children",
        "id",
        children,
        "parent_id",
        None,
        true,
        |node| {
            node.include(
                child_types,
                "type_name",
                "type_id",
                "id",
                Some("name".into()),
                false,
            );
        },
    );
    let item = repo.read_single(None).await.unwrap().unwrap();
    assert_eq!(item["type_name"], Value::from("org"));
    let children_text = item["children"].wire_text();
    let parts: Vec<_> = children_text.split('|').collect();
    assert!(parts[0].contains("type_name=home"));
    assert!(!parts[1].contains("type_name="));
}

struct CountingRepository {
    inner: Arc<dyn ReadonlyRepository>,
    reads: Mutex<u32>,
    read_singles: Mutex<u32>,
}

impl IRepository for CountingRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl ReadonlyRepository for CountingRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        *self.read_singles.lock().unwrap() += 1;
        self.inner.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        *self.read_singles.lock().unwrap() += 1;
        self.inner.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_where(where_clause).await
    }
}

#[tokio::test]
async fn hashset_uses_one_read_per_edge() {
    let types = Arc::new(CountingRepository {
        inner: Arc::new(items_repository::ItemsRepository::new(vec![
            record(&[("id", Value::Int(10)), ("name", Value::from("person"))]),
            record(&[("id", Value::Int(11)), ("name", Value::from("org"))]),
        ])),
        reads: Mutex::new(0),
        read_singles: Mutex::new(0),
    });
    let children = Arc::new(CountingRepository {
        inner: Arc::new(items_repository::ItemsRepository::new(vec![
            record(&[("parent_id", Value::Int(1)), ("label", Value::from("a"))]),
            record(&[("parent_id", Value::Int(2)), ("label", Value::from("b"))]),
        ])),
        reads: Mutex::new(0),
        read_singles: Mutex::new(0),
    });
    let parents = Arc::new(items_repository::ItemsRepository::new(vec![
        record(&[("id", Value::Int(1)), ("type_id", Value::Int(10))]),
        record(&[("id", Value::Int(2)), ("type_id", Value::Int(11))]),
    ]));
    let mut repo = EnrichedReadonlyRepository::with_strategy(
        parents,
        Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy),
    );
    repo.add_enriched(
        "type_name",
        "type_id",
        types.clone(),
        "id",
        Some("name".into()),
        false,
    )
    .add_enriched("children", "id", children.clone(), "parent_id", None, true);
    repo.read(None).await.unwrap();
    assert_eq!(*types.reads.lock().unwrap(), 1);
    assert_eq!(*types.read_singles.lock().unwrap(), 0);
    assert_eq!(*children.reads.lock().unwrap(), 1);
    assert_eq!(*children.read_singles.lock().unwrap(), 0);
}

#[tokio::test]
async fn flat_to_nested_collapses_join_rows() {
    let source = Arc::new(items_repository::ItemsRepository::new(vec![
        record(&[("id", Value::Int(1)), ("type_name", Value::from("person"))]),
        record(&[("id", Value::Int(2)), ("type_name", Value::from("org"))]),
    ]));
    let mapper = FlatToNestedMapper::new(TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("type_name".into(), TypeMapField::column("type_name")),
        ],
    ));
    let strategy = FlatToNestedReadonlyEnrichmentStrategy::new(
        source,
        mapper,
        |mapped| {
            mapped
                .into_iter()
                .filter_map(|(key, value)| match value {
                    crate::mappers::MappedValue::Cell(cell) => Some((key, cell)),
                    _ => None,
                })
                .collect()
        },
        None,
    );
    let repo = EnrichedReadonlyRepository::with_strategy(
        Arc::new(items_repository::ItemsRepository::new(vec![])),
        Arc::new(strategy),
    );
    let items = repo.read(None).await.unwrap();
    assert_eq!(items[0]["id"], Value::Int(1));
    assert_eq!(items[0]["type_name"], Value::from("person"));
    assert_eq!(items[1]["id"], Value::Int(2));
    assert_eq!(items[1]["type_name"], Value::from("org"));
}

#[tokio::test]
async fn contacts_service_fills_names_and_child_lists() {
    let contacts = Arc::new(list_repository::ListRepository::new(vec![]));
    let types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(2)),
        ("name", Value::from("person")),
    ])]));
    let sources = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(3)),
        ("name", Value::from("import")),
    ])]));
    let address_types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(1)),
        ("name", Value::from("Home")),
    ])]));
    let phone_types = Arc::new(items_repository::ItemsRepository::new(vec![record(&[
        ("id", Value::Int(2)),
        ("name", Value::from("Work")),
    ])]));
    let addresses = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(11)),
        ("contact_id", Value::Int(9)),
        ("line1", Value::from("other")),
    ])]));
    let phones = Arc::new(list_repository::ListRepository::new(vec![]));
    let service = contact::ContactsService::new(
        contacts,
        types,
        sources,
        address_types,
        phone_types,
        addresses,
        phones,
    );
    service
        .create(&record(&[
            ("contact_type_id", Value::Int(2)),
            ("contact_source_id", Value::Int(3)),
            ("first_name", Value::from("Ada")),
            (
                "addresses",
                encode_rows(&[record(&[
                    ("line1", Value::from("1 Main")),
                    ("city", Value::from("Paris")),
                    ("address_type_id", Value::Int(1)),
                ])]),
            ),
            (
                "phones",
                encode_rows(&[record(&[
                    ("number", Value::from("555")),
                    ("phone_type_id", Value::Int(2)),
                ])]),
            ),
        ]))
        .await
        .unwrap();
    let contact = service.read(None).await.unwrap().into_iter().next().unwrap();
    assert_eq!(contact["contact_type_name"], Value::from("person"));
    assert_eq!(contact["contact_source_name"], Value::from("import"));
    assert!(contact["addresses"].wire_text().contains("line1=1 Main"));
    assert!(contact["addresses"]
        .wire_text()
        .contains("address_type_name=Home"));
    assert!(!contact["addresses"].wire_text().contains("line1=other"));
    assert!(contact["phones"].wire_text().contains("number=555"));
    assert!(contact["phones"]
        .wire_text()
        .contains("phone_type_name=Work"));
}

#[tokio::test]
async fn deep_create_writes_children_stamps_fk_and_skips_select() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![]));
    let types = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(10)),
        ("name", Value::from("person")),
    ])]));
    let children = Arc::new(list_repository::ListRepository::new(vec![]));
    let mut repo = DeepCRUDRepository::new(parents.clone());
    repo.add_enriched(
        "type_name",
        "type_id",
        types.clone(),
        "id",
        Some("name".into()),
        false,
    )
    .add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    repo.create(&record(&[
        ("type_id", Value::Int(10)),
        (
            "children",
            encode_rows(&[
                record(&[("label", Value::from("a"))]),
                record(&[("label", Value::from("b"))]),
            ]),
        ),
    ]))
    .await
    .unwrap();
    let stored = children.snapshot();
    assert_eq!(stored.len(), 2);
    assert_eq!(stored[0]["parent_id"], Value::Int(1));
    assert_eq!(stored[1]["parent_id"], Value::Int(1));
    assert_eq!(types.snapshot().len(), 1);
}

#[tokio::test]
async fn deep_create_applies_generated_parent_id_before_child_fks() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![record(&[(
        "id",
        Value::Int(4),
    )])]));
    let children = Arc::new(list_repository::ListRepository::new(vec![]));
    let mut repo = DeepCRUDRepository::new(parents);
    repo.add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    repo.create(&record(&[(
        "children",
        encode_rows(&[record(&[("label", Value::from("n"))])]),
    )]))
    .await
    .unwrap();
    assert_eq!(children.snapshot()[0]["parent_id"], Value::Int(5));
}

#[tokio::test]
async fn deep_update_replace_syncs_children() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(1)),
        ("type_id", Value::Int(10)),
    ])]));
    let children = Arc::new(list_repository::ListRepository::new(vec![
        record(&[
            ("id", Value::Int(1)),
            ("parent_id", Value::Int(1)),
            ("label", Value::from("a")),
        ]),
        record(&[
            ("id", Value::Int(2)),
            ("parent_id", Value::Int(1)),
            ("label", Value::from("b")),
        ]),
    ]));
    let mut repo = DeepCRUDRepository::new(parents);
    repo.add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    repo.update(
        &record(&[
            ("id", Value::Int(1)),
            (
                "children",
                encode_rows(&[
                    record(&[("id", Value::Int(1)), ("label", Value::from("aa"))]),
                    record(&[("label", Value::from("c"))]),
                ]),
            ),
        ]),
        None,
    )
    .await
    .unwrap();
    let stored = children.snapshot();
    assert_eq!(stored.len(), 2);
    assert_eq!(
        stored
            .iter()
            .find(|row| row["id"] == Value::Int(1))
            .unwrap()["label"],
        Value::from("aa")
    );
    assert_eq!(
        stored
            .iter()
            .find(|row| row["id"] != Value::Int(1))
            .unwrap()["label"],
        Value::from("c")
    );
}

#[tokio::test]
async fn deep_delete_cascades_children_then_parent() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![record(&[(
        "id",
        Value::Int(1),
    )])]));
    let children = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(3)),
        ("parent_id", Value::Int(1)),
        ("label", Value::from("a")),
    ])]));
    let notes = Arc::new(list_repository::ListRepository::new(vec![
        record(&[
            ("id", Value::Int(8)),
            ("child_id", Value::Int(3)),
            ("text", Value::from("n")),
        ]),
        record(&[
            ("id", Value::Int(9)),
            ("child_id", Value::Int(9)),
            ("text", Value::from("other")),
        ]),
    ]));
    let mut repo = DeepCRUDRepository::new(parents.clone());
    repo.add_enriched_repo_with(
        "children",
        "id",
        children.clone(),
        "parent_id",
        None,
        true,
        |node| {
            node.include_repo(notes.clone(), "notes", "id", "child_id", None, true);
        },
    );
    repo.delete(&record(&[("id", Value::Int(1))]), None)
        .await
        .unwrap();
    assert!(parents.snapshot().is_empty());
    assert!(children.snapshot().is_empty());
    assert_eq!(notes.snapshot()[0]["text"], Value::from("other"));
}

#[tokio::test]
async fn deep_nested_many_writes_and_nested_select_is_skipped() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![]));
    let children = Arc::new(list_repository::ListRepository::new(vec![]));
    let notes = Arc::new(list_repository::ListRepository::new(vec![]));
    let child_types = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(3)),
        ("name", Value::from("home")),
    ])]));
    let mut repo = DeepCRUDRepository::new(parents);
    repo.add_enriched_repo_with(
        "children",
        "id",
        children.clone(),
        "parent_id",
        None,
        true,
        |node| {
            node.include(
                child_types.clone(),
                "type_name",
                "type_id",
                "id",
                Some("name".into()),
                false,
            );
            node.include_repo(notes.clone(), "notes", "id", "child_id", None, true);
        },
    );
    repo.create(&record(&[(
        "children",
        encode_rows(&[record(&[
            ("type_id", Value::Int(3)),
            ("label", Value::from("a")),
            (
                "notes",
                encode_rows(&[record(&[("text", Value::from("hi"))])]),
            ),
        ])]),
    )]))
    .await
    .unwrap();
    let notes = notes.snapshot();
    assert_eq!(notes[0]["text"], Value::from("hi"));
    assert_eq!(notes[0]["child_id"], children.snapshot()[0]["id"]);
    assert_eq!(child_types.snapshot()[0]["name"], Value::from("home"));
}

#[tokio::test]
async fn deep_many_readonly_via_throws() {
    let mut repo = DeepCRUDRepository::new(Arc::new(list_repository::ListRepository::new(vec![])));
    repo.add_enriched(
        "children",
        "id",
        Arc::new(items_repository::ItemsRepository::new(vec![])),
        "parent_id",
        None,
        true,
    );
    let error = repo
        .create(&record(&[(
            "children",
            encode_rows(&[record(&[("label", Value::from("a"))])]),
        )]))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("children"));
}

#[tokio::test]
async fn deep_get_still_enriches() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(1)),
        ("type_id", Value::Int(10)),
    ])]));
    let types = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("id", Value::Int(10)),
        ("name", Value::from("org")),
    ])]));
    let children = Arc::new(list_repository::ListRepository::new(vec![record(&[
        ("parent_id", Value::Int(1)),
        ("label", Value::from("a")),
    ])]));
    let mut repo = DeepCRUDRepository::new(parents);
    repo.add_enriched(
        "type_name",
        "type_id",
        types,
        "id",
        Some("name".into()),
        false,
    )
    .add_enriched_repo("children", "id", children, "parent_id", None, true);
    let item = repo.read_single(None).await.unwrap().unwrap();
    assert_eq!(item["type_name"], Value::from("org"));
    assert!(item["children"].wire_text().contains("label=a"));
}

#[tokio::test]
async fn bulk_create_writes_parents_and_children() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![]));
    let children = Arc::new(list_repository::ListRepository::new(vec![]));
    let mut repo = BulkCRUDRepository::new(parents.clone());
    repo.add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    let response = repo
        .create_bulk(&[
            record(&[(
                "children",
                encode_rows(&[record(&[("label", Value::from("a"))])]),
            )]),
            record(&[(
                "children",
                encode_rows(&[
                    record(&[("label", Value::from("b"))]),
                    record(&[("label", Value::from("c"))]),
                ]),
            )]),
        ])
        .await
        .unwrap();
    assert_eq!(parents.snapshot().len(), 2);
    let stored = children.snapshot();
    assert_eq!(stored.len(), 3);
    assert_eq!(stored[0]["parent_id"], Value::Int(1));
    assert_eq!(stored[1]["parent_id"], Value::Int(2));
    assert_eq!(stored[2]["parent_id"], Value::Int(2));
    assert_eq!(response.rows_affected, 2);
}

#[tokio::test]
async fn bulk_update_replace_syncs_each_parent() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![
        record(&[("id", Value::Int(1))]),
        record(&[("id", Value::Int(2))]),
    ]));
    let children = Arc::new(list_repository::ListRepository::new(vec![
        record(&[
            ("id", Value::Int(1)),
            ("parent_id", Value::Int(1)),
            ("label", Value::from("a")),
        ]),
        record(&[
            ("id", Value::Int(2)),
            ("parent_id", Value::Int(2)),
            ("label", Value::from("b")),
        ]),
    ]));
    let mut repo = BulkCRUDRepository::new(parents);
    repo.add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    repo.update_bulk(
        &[
            record(&[
                ("id", Value::Int(1)),
                (
                    "children",
                    encode_rows(&[record(&[
                        ("id", Value::Int(1)),
                        ("label", Value::from("aa")),
                    ])]),
                ),
            ]),
            record(&[
                ("id", Value::Int(2)),
                (
                    "children",
                    encode_rows(&[record(&[("label", Value::from("c"))])]),
                ),
            ]),
        ],
        None,
    )
    .await
    .unwrap();
    let stored = children.snapshot();
    assert_eq!(stored.len(), 2);
    assert_eq!(
        stored
            .iter()
            .find(|row| row["parent_id"] == Value::Int(1))
            .unwrap()["label"],
        Value::from("aa")
    );
    assert_eq!(
        stored
            .iter()
            .find(|row| row["parent_id"] == Value::Int(2))
            .unwrap()["label"],
        Value::from("c")
    );
}

#[tokio::test]
async fn bulk_delete_cascades_each_parent() {
    let parents = Arc::new(list_repository::ListRepository::new(vec![
        record(&[("id", Value::Int(1))]),
        record(&[("id", Value::Int(2))]),
    ]));
    let children = Arc::new(list_repository::ListRepository::new(vec![
        record(&[
            ("id", Value::Int(1)),
            ("parent_id", Value::Int(1)),
            ("label", Value::from("a")),
        ]),
        record(&[
            ("id", Value::Int(2)),
            ("parent_id", Value::Int(2)),
            ("label", Value::from("b")),
        ]),
        record(&[
            ("id", Value::Int(3)),
            ("parent_id", Value::Int(9)),
            ("label", Value::from("keep")),
        ]),
    ]));
    let mut repo = BulkCRUDRepository::new(parents.clone());
    repo.add_enriched_repo("children", "id", children.clone(), "parent_id", None, true);
    repo.delete_bulk(
        &[
            record(&[("id", Value::Int(1))]),
            record(&[("id", Value::Int(2))]),
        ],
        None,
    )
    .await
    .unwrap();
    assert!(parents.snapshot().is_empty());
    assert_eq!(children.snapshot()[0]["label"], Value::from("keep"));
}
