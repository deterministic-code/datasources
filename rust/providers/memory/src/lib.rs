mod converters;
mod command;
mod connection;
mod datasource;
mod filter;
mod repository;
mod store;
mod transaction;

pub use command::MemoryCommand;
pub use connection::MemoryConnection;
pub use datasource::MemoryDatasource;
pub use repository::MemoryRepository;
pub use store::{MemoryData, MemoryStore};
pub use transaction::MemoryTransaction;

#[cfg(test)]
pub(crate) use filter::{json_to_value, matches, parse_filter};

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        create, register, reset, Command, Connection, DatasourceFactory, Parameter,
        Row, ReadonlyRepository, Repository, TextQuery, TextWhere, Value,
    };
    use serde_json::Value as JsonValue;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn rec(id: i64, name: &str) -> Row {
        let mut r = Row::new();
        r.insert("id".into(), Value::Int(id));
        r.insert("name".into(), Value::from(name));
        r
    }

    #[tokio::test]
    async fn memory_crud_and_transactions() {
        let store = Arc::new(MemoryStore::new());
        store.seed(&MemoryData::Rows(vec![rec(1, "a")]), "");
        store.seed(&MemoryData::Rows(vec![rec(2, "b")]), "");
        let mut named = HashMap::new();
        named.insert("users".into(), vec![rec(3, "c")]);
        named.insert("posts".into(), vec![rec(4, "d")]);
        store.seed(&MemoryData::Collections(named.clone()), "");
        store.seed(&MemoryData::Collections(named), "");
        assert_eq!(store.collection("", |items| items.len()), 1);
        assert_eq!(store.collection("users", |items| items[0]["id"].clone()), Value::Int(3));

        let conn = Arc::new(MemoryConnection::new(
            store.clone(),
            "",
            Some(MemoryData::Rows(vec![rec(8, "x")])),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.collection_name(), "");
        let repo = MemoryRepository::new(conn.clone());
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

        let command = MemoryCommand::new(conn.clone());
        command
            .execute(Some("insert"), Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[
                    Parameter::new("id", 5_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":5}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute(Some("update"), Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute(
                Some("update"),
                Some(&[Parameter::new("$where", 1_i64)]),
            )
            .await
            .unwrap();
        command
            .execute(Some("delete"), Some(&[Parameter::new("id", 5_i64)]))
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
        assert!(conn.in_transaction());
        repo.add(&rec(9, "t")).await.unwrap();
        txn.abort().await.unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        repo.add(&rec(8, "c")).await.unwrap();
        txn.commit().await.unwrap();
        let _ = conn.start_transaction(None).await.unwrap();
        conn.finish_abort();
        assert!(!conn.in_transaction());
        conn.finish_abort();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());

        let named_conn = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "users",
            Some(MemoryData::Collections({
                let mut map = HashMap::new();
                map.insert("users".into(), vec![rec(1, "a")]);
                map
            })),
        ));
        named_conn.open().await.unwrap();
        MemoryRepository::new(named_conn.clone())
            .add(&rec(2, "b"))
            .await
            .unwrap();

        reset();
        register(MemoryDatasource::instance());
        let ds = create("memory", None).unwrap();
        assert_eq!(ds.provider(), "memory");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute(Some("insert"), None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let bound = ds.create_command(Some(ds.create_connection().await.unwrap()));
        assert!(bound.execute(Some("insert"), None).await.is_err());
        assert!(bound.execute_reader(None, None).await.is_err());
        ds.create_repository(None);
        ds.create_repository(Some(ds.create_connection().await.unwrap()));
        let factory = MemoryDatasource::default();
        assert_eq!(DatasourceFactory::provider(&factory), "memory");
        DatasourceFactory::create(&factory, None);
        assert_eq!(create("memory", None).unwrap().provider(), "memory");

        assert!(parse_filter(None).unwrap().is_empty());
        assert!(parse_filter(Some("   ")).unwrap().is_empty());
        assert!(parse_filter(Some("[]")).is_err());
        assert!(parse_filter(Some("not-json")).is_err());
        assert!(!matches(&Row::new(), &{
            let mut f = Row::new();
            f.insert("id".into(), Value::Int(1));
            f
        }));
        json_to_value(&JsonValue::Null);
        json_to_value(&JsonValue::Bool(true));
        json_to_value(&serde_json::json!(1));
        json_to_value(&JsonValue::Number(serde_json::Number::from_f64(1.25).unwrap()));
        json_to_value(&serde_json::json!("x"));
        json_to_value(&serde_json::json!([1]));
        if let JsonValue::Number(n) = serde_json::json!(1) {
            json_to_value(&JsonValue::Number(n));
        }
        json_to_value(&JsonValue::Number(serde_json::Number::from(u64::MAX)));

        let closed = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "",
            None,
        ));
        assert!(MemoryCommand::new(closed.clone())
            .execute_reader(None, None)
            .await
            .is_err());
        assert!(MemoryCommand::new(closed.clone())
            .execute(Some("insert"), None)
            .await
            .is_err());
        assert!(MemoryRepository::new(closed.clone())
            .add(&rec(1, "a"))
            .await
            .is_err());
        assert!(MemoryRepository::new(closed.clone())
            .update(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(MemoryRepository::new(closed)
            .delete(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(MemoryConnection::new(Arc::new(MemoryStore::new()), "", None)
            .start_transaction(None)
            .await
            .is_err());

        let empty = Arc::new(MemoryConnection::new(
            Arc::new(MemoryStore::new()),
            "",
            None,
        ));
        empty.open().await.unwrap();
        empty.finish_commit();
        empty.finish_abort();
    }
}

pub use converters::{register as register_converters, types as converter_types};
