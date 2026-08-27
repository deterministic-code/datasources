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
    let empty = ArrayReader::new(vec![]);
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
}

#[tokio::test]
async fn middleware_paths() {
    let host = MiddlewareHost::new();
    host.use_middleware(None, None);
    assert_eq!(host.run("close", None, || async { Ok(1) }).await.unwrap(), 1);

    let order = Arc::new(Mutex::new(Vec::new()));
    let seen = order.clone();
    host.use_middleware(
        Some(hook({
            let seen = seen.clone();
            move |ctx| {
                let seen = seen.clone();
                async move {
                    seen.lock().unwrap().push(format!("b:{}", ctx.operation));
                    Ok(())
                }
            }
        })),
        Some(hook({
            let seen = seen.clone();
            move |ctx| {
                let seen = seen.clone();
                async move {
                    seen.lock().unwrap().push(format!("a:{}", ctx.operation));
                    assert!(ctx.error.is_none());
                    Ok(())
                }
            }
        })),
    );
    host.run("open", None, || async { Ok(()) }).await.unwrap();
    assert_eq!(*order.lock().unwrap(), vec!["b:open", "a:open"]);

    let blocked = MiddlewareHost::new();
    let ran = Arc::new(Mutex::new(false));
    blocked.use_middleware(
        Some(hook(|_| async { Err(DatasourceError::message("blocked")) })),
        Some(hook({
            let ran = ran.clone();
            move |_| {
                let ran = ran.clone();
                async move {
                    *ran.lock().unwrap() = true;
                    Ok(())
                }
            }
        })),
    );
    assert!(blocked
        .run("execute", None, || async {
            *ran.lock().unwrap() = true;
            Ok(1)
        })
        .await
        .is_err());
    assert!(!*ran.lock().unwrap());

    let failed = MiddlewareHost::new();
    let captured = Arc::new(Mutex::new(None));
    failed.use_middleware(
        None,
        Some(hook({
            let captured = captured.clone();
            move |ctx| {
                let captured = captured.clone();
                async move {
                    *captured.lock().unwrap() = ctx.error.clone();
                    Ok(())
                }
            }
        })),
    );
    assert!(failed
        .run("abort", None, || async { Err::<(), _>(DatasourceError::message("fail-op")) })
        .await
        .is_err());
    assert_eq!(*captured.lock().unwrap(), Some("fail-op".into()));

    let extras = MiddlewareHost::new();
    extras
        .run(
            "execute",
            Some((Some("insert".into()), Some(vec![Parameter::new("id", 1_i64)]))),
            || async { Ok(()) },
        )
        .await
        .unwrap();
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
        .start_transaction()
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
    let txn = StubTransaction::new("text");
    assert!(txn.commit().await.is_err());
    assert!(txn.abort().await.is_err());
    let reader = StubReader::new("text");
    assert!(reader.current().is_none());
    assert!(reader.next().await.is_err());
    let conn = StubConnection::new("text");
    conn.use_middleware(None, None);
    let cmd = StubCommand::new("text");
    cmd.use_middleware(None, None);
    txn.use_middleware(None, None);
}

#[tokio::test]
async fn published_mocks() {
    let connection = MockConnection::new();
    connection.open().await.unwrap();
    assert!(connection.opened());
    assert_eq!(connection.open_calls(), 1);
    connection.start_transaction().await.unwrap();
    connection.close().await.unwrap();
    connection.set_open_error("open-fail");
    assert!(connection.open().await.is_err());
    connection.set_close_error("close-fail");
    assert!(connection.close().await.is_err());
    connection.set_start_error("tx-fail");
    assert!(connection.start_transaction().await.is_err());
    let custom = Arc::new(MockTransaction::new_unbound());
    let with_txn = MockConnection::with_transaction(custom);
    with_txn.start_transaction().await.unwrap();

    let txn = MockTransaction::new_unbound();
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
    DatasourceFactory::create(&ds, Some("{}"));

    let q = MockQuery::new("q");
    let w = MockWhere::new("w");
    assert_eq!(q.get_text(), "q");
    assert_eq!(w.get_text(), "w");
    assert_eq!(q.calls(), 1);
    assert_eq!(w.calls(), 1);
    assert_eq!(TextWhere::new("x").get_text(), "x");

    let conn = MockConnection::new();
    conn.use_middleware(None, None);
    let seen = Arc::new(Mutex::new(false));
    conn.use_middleware(
        Some(hook({
            let seen = seen.clone();
            move |_| {
                let seen = seen.clone();
                async move {
                    *seen.lock().unwrap() = true;
                    Ok(())
                }
            }
        })),
        None,
    );
    conn.open().await.unwrap();
    assert!(*seen.lock().unwrap());
}

#[test]
fn middleware_context_constructor() {
    let ctx = MiddlewareContext::new("open");
    assert_eq!(ctx.operation, "open");
    assert!(ctx.text.is_none());
}
