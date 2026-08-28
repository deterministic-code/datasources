use crate::converters::{identity_provider_types, Cells};
use crate::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn record(pairs: &[(&str, Value)]) -> Record {
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
    assert_eq!(IsolationLevel::from_name("other"), IsolationLevel::Serializable);
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
    assert_eq!(parameters_to_map(None).len(), 0);
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
    assert_eq!(identity_where_text(&HashMap::new()), "1 = 1");
}

#[tokio::test]
async fn readers() {
    let empty = ArrayReader::<Record>::new(vec![]);
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
        fn current(&self) -> Option<Record> {
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
        fn current(&self) -> Option<Record> {
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
    async fn before_query(
        &self,
        query: &str,
        _params: Option<&[Parameter]>,
    ) -> Result<Option<(String, Option<Vec<Parameter>>)>> {
        (self.before)(query)
    }

    async fn after_query(
        &self,
        query: &str,
        _params: Option<&[Parameter]>,
        _results: &[Record],
        _elapsed_ms: f64,
        error: Option<&DatasourceError>,
    ) -> Result<()> {
        (self.after)(query, error)
    }
}

#[tokio::test]
async fn middleware_paths() {
    let host = CommandMiddlewareHost::new();
    assert_eq!(
        host.run(None, None, |_q, _p| async { Ok((1, vec![])) })
            .await
            .unwrap(),
        1
    );

    let order = Arc::new(Mutex::new(Vec::new()));
    let seen = order.clone();
    host.use_middleware(Arc::new(RecordingMiddleware {
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
    host.run(Some("SELECT 1"), None, move |query, _p| {
        let used_clone = used_clone.clone();
        async move {
            *used_clone.lock().unwrap() = query;
            Ok(((), vec![Record::new()]))
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
    blocked.use_middleware(Arc::new(RecordingMiddleware {
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
        .run(Some("select"), None, {
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
    failed.use_middleware(Arc::new(RecordingMiddleware {
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
        .run(Some("select"), None, |_q, _p| async {
            Err::<((), Vec<Record>), _>(DatasourceError::message("fail-op"))
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
    host.use_middleware(Arc::new(middleware));
    host.run(
        Some(r#"SELECT * FROM "widget" ORDER BY "id""#),
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
    host.use_middleware(Arc::new(middleware));
    host.run(Some("SELECT 1\n"), None, |_q, _p| async { Ok(((), vec![])) })
        .await
        .unwrap();
    let logged = lines.lock().unwrap()[0].clone();
    let signature = logged.replacen("[2026-08-28T01:54:12.101Z]-", "", 1);
    assert_eq!(signature, "[datasource][Start]-[SELECT 1]");
}

#[test]
fn registry() {
    reset();
    assert!(registered().is_empty());
    let factory = Arc::new(MockDatasource::new("json"));
    register(factory);
    assert_eq!(registered(), vec!["json".to_string()]);
    let created = create("json", Some("{}")).unwrap();
    assert_eq!(created.provider(), "json");
    reset();
    assert!(create("json", None).is_err());
}

#[tokio::test]
async fn stubs_throw() {
    let ds = create_stub_datasource("text");
    assert_eq!(Datasource::provider(&*ds), "text");
    let created = DatasourceFactory::create(&*ds, None);
    assert_eq!(created.provider(), "text");
    assert!(ds.create_connection().await.unwrap().open().await.is_err());
    assert!(ds.create_connection().await.unwrap().close().await.is_err());
    assert!(ds
        .create_connection()
        .await
        .unwrap()
        .start_transaction(None)
        .await
        .is_err());
    let command = ds.create_command(None);
    assert!(command.execute(None, None).await.is_err());
    assert!(command
        .execute_reader(Some("select"), Some(&[Parameter::new("id", 1_i64)]))
        .await
        .is_err());
    let repo = ds.create_repository(None);
    let empty = Record::new();
    assert!(repo.get_single(None).await.is_err());
    assert!(repo.get(None).await.is_err());
    assert!(repo.add(&empty).await.is_err());
    assert!(repo.update(&empty, None).await.is_err());
    assert!(repo.delete(&empty, None).await.is_err());
    let readonly = ds.create_readonly_repository(None);
    assert!(readonly.get_single(None).await.is_err());
    assert!(readonly.get(None).await.is_err());
    let txn = StubTransaction::new("text");
    assert!(txn.commit().await.is_err());
    assert!(txn.abort().await.is_err());
    let reader = StubReader::<Record>::new("text");
    assert!(reader.current().is_none());
    assert!(reader.next().await.is_err());
    let conn = StubConnection::new("text");
    let cmd = StubCommand::new("text");
    cmd.use_middleware(Arc::new(RecordingMiddleware {
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
    assert!(connection.opened());
    assert_eq!(connection.open_calls(), 1);
    connection
        .start_transaction(Some(IsolationLevel::ReadUncommitted))
        .await
        .unwrap();
    assert_eq!(
        connection.last_isolation(),
        IsolationLevel::ReadUncommitted
    );
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
    command.execute(Some("insert"), None).await.unwrap();
    command
        .execute_reader(None, Some(&[Parameter::new("id", 1_i64)]))
        .await
        .unwrap();
    command.set_execute_error("exec");
    assert!(command.execute(None, None).await.is_err());
    command.set_reader_error("read");
    assert!(command.execute_reader(None, None).await.is_err());

    let reader = MockReader::new(vec![record(&[("id", Value::Int(1))])]);
    assert!(reader.next().await.unwrap());
    reader.set_error("next");
    assert!(reader.next().await.is_err());

    let repo = MockRepository::new();
    assert!(repo.get_single(None).await.unwrap().is_none());
    repo.items
        .lock()
        .unwrap()
        .push(record(&[("id", Value::Int(1))]));
    assert!(repo
        .get_single(Some(&TextQuery::new("id=1")))
        .await
        .unwrap()
        .is_some());
    assert_eq!(repo.get(Some(&MockQuery::new("all"))).await.unwrap().len(), 1);
    repo.add(&record(&[("id", Value::Int(2))])).await.unwrap();
    repo.update(&record(&[("id", Value::Int(2))]), Some(&MockWhere::new("id=2")))
        .await
        .unwrap();
    repo.delete(&record(&[("id", Value::Int(2))]), None)
        .await
        .unwrap();
    repo.set_get_single_error("gs");
    assert!(repo.get_single(None).await.is_err());
    repo.set_get_error("g");
    assert!(repo.get(None).await.is_err());
    repo.set_add_error("ad");
    assert!(repo.add(&Record::new()).await.is_err());
    repo.set_update_error("up");
    assert!(repo.update(&Record::new(), None).await.is_err());
    repo.set_delete_error("del");
    assert!(repo.delete(&Record::new(), None).await.is_err());

    let ds = MockDatasource::new("mock");
    assert_eq!(Datasource::provider(&ds), "mock");
    assert_eq!(DatasourceFactory::provider(&ds), "mock");
    ds.create_connection().await.unwrap();
    let command = ds.create_command(None);
    command.execute(None, None).await.unwrap();
    command.execute_reader(None, None).await.unwrap();
    ds.create_repository(None);
    ds.create_readonly_repository(None);
    DatasourceFactory::create(&ds, Some("{}"));

    let q = MockQuery::new("q");
    let w = MockWhere::new("w");
    assert_eq!(q.get_text(), "q");
    assert_eq!(w.get_text(), "w");
    assert_eq!(q.calls(), 1);
    assert_eq!(w.calls(), 1);
    assert_eq!(TextWhere::new("x").get_text(), "x");

    let command = MockCommand::new();
    let seen = Arc::new(Mutex::new(Vec::new()));
    command.use_middleware(Arc::new(RecordingMiddleware {
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
    command.execute(Some("insert"), None).await.unwrap();
    assert_eq!(
        *seen.lock().unwrap(),
        vec!["b:insert".to_string(), "a:insert".to_string()]
    );
}

#[test]
fn statements_and_builders() {
    let where_clause = WhereBuilder::and(vec![
        WhereBuilder::equals("id", 1_i64),
        WhereBuilder::greater_than("age", 18_i64),
    ]);
    assert_eq!(where_clause.get_text(), "id = $id AND age > $age");
    assert_eq!(where_clause.parameters().len(), 2);
    assert_eq!(where_clause.parameters()[0].value, Value::Int(1));
    assert_eq!(where_clause.parameters()[1].value, Value::Int(18));
    assert_eq!(WhereBuilder::ne("id", 2_i64).get_text(), "id <> $id");
    assert_eq!(WhereBuilder::lt("n", 0_i64).get_text(), "n < $n");
    assert_eq!(WhereBuilder::gte("n", 0_i64).get_text(), "n >= $n");
    assert_eq!(WhereBuilder::lte("n", 0_i64).get_text(), "n <= $n");
    assert_eq!(WhereBuilder::and(vec![]).get_text(), "1 = 1");
    assert_eq!(WhereBuilder::or(vec![]).get_text(), "1 = 0");
    assert_eq!(
        WhereBuilder::and(vec![WhereBuilder::equals("id", 1_i64)]).get_text(),
        "id = $id"
    );
    assert_eq!(
        WhereBuilder::or(vec![WhereBuilder::equals("id", 1_i64)]).get_text(),
        "id = $id"
    );
    assert_eq!(
        WhereBuilder::or(vec![where_clause.clone(), WhereBuilder::equals("name", "a")]).get_text(),
        "(id = $id AND age > $age) OR name = $name"
    );
    assert_eq!(
        WhereBuilder::not(WhereBuilder::equals("id", 1_i64)).get_text(),
        "NOT (id = $id)"
    );

    let fields = [
        Parameter::new("id", 1_i64),
        Parameter::new("name", "a"),
    ];
    let query = StatementBuilder::create_query("users", &["id", "name"], Some(&where_clause));
    assert_eq!(
        query.get_text(),
        "SELECT id, name FROM users WHERE id = $id AND age > $age"
    );
    assert!(query.where_clause().is_some());
    assert_eq!(
        StatementBuilder::create_query("users", &[], None).get_text(),
        "SELECT * FROM users"
    );
    assert!(
        StatementBuilder::create_query("users", &[], None)
            .where_clause()
            .is_none()
    );
    assert_eq!(
        StatementBuilder::create_query("users", &["id", "name"], None).get_text(),
        "SELECT id, name FROM users"
    );
    assert_eq!(
        StatementBuilder::create_insert("users", &fields).get_text(),
        "INSERT INTO users (id, name) VALUES ($id, $name)"
    );
    let update = StatementBuilder::create_update("users", &fields, Some(&where_clause));
    assert_eq!(
        update.get_text(),
        "UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age"
    );
    assert!(update.where_clause().is_some());
    assert!(StatementBuilder::create_update("users", &fields, None)
        .where_clause()
        .is_none());
    let delete = StatementBuilder::create_delete("users", Some(&where_clause));
    assert_eq!(
        delete.get_text(),
        "DELETE FROM users WHERE id = $id AND age > $age"
    );
    assert!(delete.where_clause().is_some());
    assert_eq!(
        StatementBuilder::create_delete("users", None).get_text(),
        "DELETE FROM users"
    );
    assert_eq!(
        StatementBuilder::create_table(
            "users",
            &[Column::new("id", "INTEGER"), Column::new("name", "TEXT")]
        )
        .get_text(),
        "CREATE TABLE users (id INTEGER, name TEXT)"
    );
    assert_eq!(
        StatementBuilder::create_stored_procedure("save", &fields).get_text(),
        "CALL save($id, $name)"
    );
    assert_eq!(
        StatementBuilder::create_stored_procedure("save", &[]).get_text(),
        "CALL save()"
    );

    let query = TextQuery::with_where("SELECT 1", TextWhere::new("id = 1"));
    assert_eq!(query.get_text(), "SELECT 1");
    assert!(query.where_clause().is_some());
    assert!(TextQuery::new("SELECT 1").where_clause().is_none());
    let mock = MockQuery::with_where("q", MockWhere::new("w"));
    assert!(mock.where_clause().is_some());
    assert_eq!(
        TextInsertStatement::new("x").get_text(),
        "x"
    );
    assert_eq!(TextUpdateStatement::new("x", None).get_text(), "x");
    assert_eq!(TextDeleteStatement::new("x", None).get_text(), "x");
    assert_eq!(TextCreateTableStatement::new("x").get_text(), "x");
    assert_eq!(TextStoredProcedureStatement::new("x").get_text(), "x");
}

#[tokio::test]
async fn table_repository() {
    reset_type_converter();
    use_type_converter("memory", identity_provider_types());
    let command = Arc::new(MockCommand::new());
    command.reader_rows.lock().unwrap().push(record(&[
        ("id", Value::Int(1)),
        ("name", Value::from("a")),
    ]));
    let mut options = TableRepositoryOptions::with_factory("users", MockConnection::new(), command.clone());
    options.provider = Some("memory".into());
    options.schema = HashMap::from([
        ("id".into(), "INTEGER".into()),
        ("name".into(), "TEXT".into()),
    ]);
    let repo = TableRepository::new(options).unwrap();
    let rows = repo.get(None).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        command.reader_logs.lock().unwrap()[0].0,
        "SELECT id, name FROM users"
    );
    assert!(repo.get_single(None).await.unwrap().is_some());

    let where_clause = WhereBuilder::equals("id", 9_i64);
    let query = StatementBuilder::create_query("users", &["id"], Some(&where_clause));
    command.reader_rows.lock().unwrap().clear();
    assert!(repo.get_single(Some(&query)).await.unwrap().is_none());
    assert_eq!(
        command.reader_logs.lock().unwrap().last().unwrap().0,
        "SELECT id FROM users WHERE id = $id"
    );

    let obj = record(&[("id", Value::Int(1)), ("name", Value::from("a"))]);
    repo.add(&obj).await.unwrap();
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

    let twice = FieldConverter::new(
        |value| Ok(value.clone()),
        |value| match value {
            Value::Int(n) => Ok(Value::Int(n * 2)),
            other => Ok(other.clone()),
        },
    );
    let generated = Arc::new(MockCommand::new());
    let mut options = TableRepositoryOptions::with_factory("users", MockConnection::new(), generated.clone());
    options.provider = Some("memory".into());
    options.id = vec!["org_id".into(), "user_id".into()];
    options.generated = true;
    options.schema = HashMap::from([
        ("org_id".into(), "INTEGER".into()),
        ("user_id".into(), "INTEGER".into()),
        ("name".into(), "TEXT".into()),
    ]);
    options.converters.insert("name".into(), twice);
    options.names.insert("name".into(), "label".into());
    let repo = TableRepository::new(options).unwrap();
    repo.add(&record(&[
        ("org_id", Value::Int(1)),
        ("user_id", Value::Int(2)),
        ("label", Value::Int(3)),
    ]))
    .await
    .unwrap();
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].0,
        "INSERT INTO users (name) VALUES ($name)"
    );
    assert_eq!(
        generated.execute_logs.lock().unwrap()[0].1[0].value,
        Value::Int(6)
    );
    repo.update(
        &record(&[
            ("org_id", Value::Int(1)),
            ("user_id", Value::Int(2)),
            ("label", Value::Int(4)),
        ]),
        None,
    )
    .await
    .unwrap();
    assert!(generated.execute_logs.lock().unwrap()[1]
        .0
        .contains("org_id = $org_id AND user_id = $user_id"));

    let command = Arc::new(MockCommand::new());
    assert!(TableRepository::new(TableRepositoryOptions::with_factory("users", MockConnection::new(), command.clone()))
        .is_err());
    let mut options = TableRepositoryOptions::with_factory("users", MockConnection::new(), command.clone());
    options.provider = Some("memory".into());
    options.schema = HashMap::from([
        ("id".into(), "INTEGER".into()),
        ("name".into(), "TEXT".into()),
    ]);
    let repo = TableRepository::new(options).unwrap();
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
    let mapping = type_mapping::<Cells>("memory", "rust").unwrap();
    let mut options = TableRepositoryOptions::with_factory("users", MockConnection::new(), command.clone());
    options.mapping = Some(mapping);
    let repo = TableRepository::new(options).unwrap();
    repo.add(&record(&[("id", Value::Int(1))])).await.unwrap();
    repo.delete(&record(&[("name", Value::from("z"))]), Some(&WhereBuilder::equals("name", "z")))
        .await
        .unwrap();
    reset_type_converter();
}
