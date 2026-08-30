mod converters;
mod command;
mod connection;
mod datasource;
mod file_store;
mod filter;
mod repository;
mod transaction;

pub use command::JsonCommand;
pub use connection::JsonConnection;
pub use datasource::JsonDatasource;
pub use file_store::{FileStore, MemoryFileStore};
pub use repository::JsonRepository;
pub use transaction::JsonTransaction;

#[cfg(test)]
pub(crate) use filter::{
    json_from_record, json_to_value, matches, parse_filter, record_from_json,
};

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        create, register, reset, Command, Connection, DatasourceFactory, Parameter,
        Row, ReadonlyRepository, Repository, TextQuery, TextWhere, Value,
    };
    use serde_json::{Map, Value as JsonValue};
    use std::sync::Arc;

    fn rec(id: i64, name: &str) -> Row {
        let mut r = Row::new();
        r.insert("id".into(), Value::Int(id));
        r.insert("name".into(), Value::from(name));
        r
    }

    #[tokio::test]
    async fn json_crud_and_transactions() {
        let store = Arc::new(MemoryFileStore::seed(
            "db.json",
            "[{\"id\":1,\"name\":\"a\"}]",
        ));
        let conn = Arc::new(JsonConnection::new(
            "db.json",
            None,
            store.clone(),
            None,
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.path(), "db.json");
        let repo = JsonRepository::new(conn.clone());
        assert!(repo.get_single(None).await.unwrap().is_some());
        assert!(repo
            .get_single(Some(&TextQuery::new("{\"id\":9}")))
            .await
            .unwrap()
            .is_none());
        assert!(!repo.get(Some(&TextQuery::new("{\"id\":1}"))).await.unwrap().is_empty());
        assert!(repo
            .get_single_where(&TextWhere::new("{\"id\":1}"))
            .await
            .unwrap()
            .is_some());
        assert!(!repo
            .get_where(&TextWhere::new("{\"id\":1}"))
            .await
            .unwrap()
            .is_empty());
        repo.add(&rec(2, "b")).await.unwrap();
        repo.update(&rec(2, "B"), None).await.unwrap();
        repo.update(&rec(2, "C"), Some(&TextWhere::new("{\"id\":2}")))
            .await
            .unwrap();
        repo.delete(&rec(2, "C"), Some(&TextWhere::new("{\"id\":2}")))
            .await
            .unwrap();
        let command = JsonCommand::new(conn.clone());
        command
            .execute(Some("insert"), Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[
                    Parameter::new("id", 3_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":3}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute(Some("update"), Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute(Some("delete"), Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command.execute(Some("delete"), None).await.unwrap();
        assert!(command.execute(Some("nope"), None).await.is_err());
        assert!(command.execute(None, None).await.is_err());
        command
            .execute_reader(Some("{\"id\":1}"), Some(&[Parameter::new("name", "a")]))
            .await
            .unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        repo.add(&rec(9, "t")).await.unwrap();
        txn.abort().await.unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        repo.add(&rec(8, "c")).await.unwrap();
        txn.commit().await.unwrap();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());

        let memory = Arc::new(JsonConnection::new(
            "",
            Some("users".into()),
            Arc::new(MemoryFileStore::new()),
            Some(serde_json::json!({})),
        ));
        memory.open().await.unwrap();
        JsonRepository::new(memory.clone())
            .add(&rec(1, "a"))
            .await
            .unwrap();
        *memory.state.data.lock().unwrap() = JsonValue::Null;
        memory.collection_mut(|_| {});
        *memory.state.data.lock().unwrap() = JsonValue::Array(vec![]);
        memory.collection_mut(|_| {});
        let txn = memory.start_transaction(None).await.unwrap();
        txn.commit().await.unwrap();

        reset();
        register(JsonDatasource::instance());
        let ds = create("json", None).unwrap();
        assert_eq!(ds.provider(), "json");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute(Some("insert"), None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let bound = ds.create_command(Some(ds.create_connection().await.unwrap()));
        assert!(bound.execute(Some("insert"), None).await.is_err());
        assert!(bound.execute_reader(None, None).await.is_err());
        ds.create_repository(None);
        ds.create_repository(Some(ds.create_connection().await.unwrap()));
        let factory = JsonDatasource::new("", None, Arc::new(MemoryFileStore::new()), None);
        assert_eq!(DatasourceFactory::provider(&factory), "json");
        DatasourceFactory::create(&factory, None);

        let missing = MemoryFileStore::new();
        assert!(!missing.exists("x"));
        assert!(missing.read("x").is_err());
        missing.write("x", "{}").unwrap();
        assert!(missing.exists("x"));

        assert!(parse_filter(None).unwrap().is_empty());
        assert!(parse_filter(Some("   ")).unwrap().is_empty());
        assert!(parse_filter(Some("[]")).is_err());
        assert!(parse_filter(Some("not-json")).is_err());
        assert!(!matches(&JsonValue::Array(vec![]), &Map::new()));
        let _ = record_from_json(&JsonValue::Array(vec![]));
        let _ = record_from_json(&serde_json::json!({
            "n": 1.5,
            "u": null,
            "b": true,
            "s": "x",
            "o": {"a": 1}
        }));
        let _ = json_from_record(&{
            let mut r = Row::new();
            r.insert("a".into(), Value::Null);
            r.insert("b".into(), Value::Bool(true));
            r.insert("c".into(), Value::Float(1.2));
            r.insert("d".into(), Value::from("t"));
            r
        });
        let closed = Arc::new(JsonConnection::new(
            "",
            None,
            Arc::new(MemoryFileStore::new()),
            None,
        ));
        assert!(JsonCommand::new(closed.clone())
            .execute_reader(None, None)
            .await
            .is_err());
        assert!(JsonCommand::new(closed.clone())
            .execute(Some("insert"), None)
            .await
            .is_err());
        assert!(JsonRepository::new(closed.clone())
            .add(&rec(1, "a"))
            .await
            .is_err());
        assert!(JsonRepository::new(closed.clone())
            .update(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(JsonRepository::new(closed)
            .delete(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(JsonConnection::new("", None, Arc::new(MemoryFileStore::new()), None)
            .start_transaction(None)
            .await
            .is_err());

        let with_col = Arc::new(JsonConnection::new(
            "c.json",
            Some("users".into()),
            Arc::new(MemoryFileStore::seed("c.json", "{}")),
            None,
        ));
        with_col.open().await.unwrap();
        JsonCommand::new(with_col)
            .execute(Some("update"), None)
            .await
            .unwrap();

        let bad = Arc::new(JsonConnection::new(
            "bad.json",
            None,
            Arc::new(MemoryFileStore::seed("bad.json", "not-json")),
            None,
        ));
        assert!(bad.open().await.is_err());

        let empty_file = Arc::new(JsonConnection::new(
            "none.json",
            None,
            Arc::new(MemoryFileStore::new()),
            None,
        ));
        empty_file.open().await.unwrap();

        json_to_value(&JsonValue::Number(serde_json::Number::from_f64(1.25).unwrap()));
        json_to_value(&serde_json::json!([1]));
        if let JsonValue::Number(n) = serde_json::json!(1) {
            json_to_value(&JsonValue::Number(n));
        }
    }
}

pub use converters::{register as register_converters, types as converter_types};
