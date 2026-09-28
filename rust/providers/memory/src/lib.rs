mod command;
mod connection;
mod filter;
mod provider;
mod store;
mod transaction;

pub use provider::MemoryProvider;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::MemoryCommand;
    use crate::connection::MemoryConnection;
    use crate::filter::{json_to_value, matches, parse_filter};
    use crate::store::{MemoryData, MemoryStore};
    use datasources::{
        converters::Cells, create, register, reset, CRUDRepository, CRUDRepositoryOptions, Command,
        Connection, ICRUDRepository, IReadonlyRepository, IsolationLevel, Parameter, Row,
        SelectStatement, Value, WhereText,
    };
    use serde_json::Value as JsonValue;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn crud(conn: Arc<MemoryConnection>) -> CRUDRepository {
        let mut options = CRUDRepositoryOptions::default();
        options.ids = vec![];
        CRUDRepository::new(conn, options).expect("repo")
    }

    fn rec(id: i64, name: &str) -> Row {
        let mut r = Row::new();
        r.insert("id".into(), Value::Int(id));
        r.insert("name".into(), Value::from(name));
        r
    }

    #[test]
    fn register_and_create() {
        reset();
        register("memory", Arc::new(MemoryProvider));
        let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())]))).unwrap();
        assert_eq!(ds.provider(), "memory");
    }

    #[tokio::test]
    async fn open_and_close() {
        let connection = MemoryConnection::from_options(None);
        connection.open().await.unwrap();
        connection.close().await.unwrap();
        assert_eq!(connection.is_open(), false);
    }

    #[tokio::test]
    async fn start_a_transaction() {
        let connection = MemoryConnection::from_options(None);
        connection.open().await.unwrap();
        let txn = connection.start_transaction(None).await.unwrap();
        assert_eq!(txn.isolation(), IsolationLevel::Serializable);
    }

    #[tokio::test]
    async fn commit() {
        let connection = MemoryConnection::from_options(None);
        connection.open().await.unwrap();
        let txn = connection.start_transaction(None).await.unwrap();
        txn.commit().await.unwrap();
        assert_eq!(txn.isolation(), IsolationLevel::Serializable);
    }

    #[tokio::test]
    async fn abort() {
        let connection = MemoryConnection::from_options(None);
        connection.open().await.unwrap();
        let txn = connection.start_transaction(None).await.unwrap();
        txn.abort().await.unwrap();
        assert_eq!(txn.isolation(), IsolationLevel::Serializable);
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
        assert_eq!(
            store.collection("users", |items| items[0]["id"].clone()),
            Value::Int(3)
        );

        let conn = Arc::new(MemoryConnection::new(
            store.clone(),
            "",
            Some(MemoryData::Rows(vec![rec(8, "x")])),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.collection_name(), "");
        let repo = crud(conn.clone());
        assert!(repo.read_single(None, None).await.unwrap().is_some());
        assert!(repo
            .read_single(Some(&SelectStatement::text("{\"id\":9}")), None)
            .await
            .unwrap()
            .is_none());
        assert!(!repo
            .read(Some(&SelectStatement::text("{\"id\":1}")), None)
            .await
            .unwrap()
            .is_empty());
        assert!(repo
            .read_single(Some(&WhereText::new("{\"id\":1}")), None)
            .await
            .unwrap()
            .is_some());
        assert!(!repo
            .read(Some(&WhereText::new("{\"id\":1}")), None)
            .await
            .unwrap()
            .is_empty());
        let created = repo.create(&mut rec(2, "b")).await.unwrap();
        assert_eq!(created.get("id"), Some(&Value::Int(2)));
        let updated = repo.update(&rec(2, "B"), None).await.unwrap();
        assert_eq!(updated.rows_affected, 0);
        let updated_by_where = repo
            .update(&rec(2, "C"), Some(&WhereText::new("{\"id\":2}")))
            .await
            .unwrap();
        assert_eq!(updated_by_where.rows_affected, 1);
        let removed = repo
            .delete(&rec(2, "C"), Some(&WhereText::new("{\"id\":2}")))
            .await
            .unwrap();
        assert_eq!(removed.rows_affected, 1);

        let command = MemoryCommand::new(conn.clone());
        command
            .execute("insert", Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute(
                "update",
                Some(&[
                    Parameter::new("id", 5_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":5}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute("update", Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command
            .execute("update", Some(&[Parameter::new("$where", 1_i64)]))
            .await
            .unwrap();
        command
            .execute("delete", Some(&[Parameter::new("id", 5_i64)]))
            .await
            .unwrap();
        command.execute("delete", None).await.unwrap();
        assert!(command.execute("nope", None).await.is_err());
        assert!(command.execute("", None).await.is_err());
        command
            .execute_reader("{\"id\":1}", Some(&[Parameter::new("name", "a")]))
            .await
            .unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        assert!(conn.in_transaction());
        let rolled_back = repo.create(&mut rec(9, "t")).await.unwrap();
        assert_eq!(rolled_back.get("id"), Some(&Value::Int(9)));
        txn.abort().await.unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        let committed = repo.create(&mut rec(8, "c")).await.unwrap();
        assert_eq!(committed.get("id"), Some(&Value::Int(8)));
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
        crud(named_conn.clone())
            .create(&mut rec(2, "b"))
            .await
            .unwrap();

        reset();
        register("memory", Arc::new(MemoryProvider));
        let ds = create(Some(&HashMap::from([("provider".into(), "memory".into())]))).unwrap();
        assert_eq!(ds.provider(), "memory");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("insert", None).await.is_err());
        assert!(command.execute_reader("", None).await.is_err());
        let bound = ds.create_command(Some(ds.create_connection().await.unwrap()));
        assert!(bound.execute("insert", None).await.is_err());
        assert!(bound.execute_reader("", None).await.is_err());
        ds.create_repository::<Cells>(None, None);
        ds.create_repository::<Cells>(Some(ds.create_connection().await.unwrap()), None);
        assert_eq!(
            create(Some(&HashMap::from([("provider".into(), "memory".into())])))
                .unwrap()
                .provider(),
            "memory"
        );

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
        json_to_value(&JsonValue::Number(
            serde_json::Number::from_f64(1.25).unwrap(),
        ));
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
            .execute_reader("", None)
            .await
            .is_err());
        assert!(MemoryCommand::new(closed.clone())
            .execute("insert", None)
            .await
            .is_err());
        assert!(crud(closed.clone()).create(&mut rec(1, "a")).await.is_err());
        assert!(crud(closed.clone())
            .update(&rec(1, "a"), None)
            .await
            .is_err());
        assert!(crud(closed).delete(&rec(1, "a"), None).await.is_err());
        assert!(
            MemoryConnection::new(Arc::new(MemoryStore::new()), "", None)
                .start_transaction(None)
                .await
                .is_err()
        );

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
