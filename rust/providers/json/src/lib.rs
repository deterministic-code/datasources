mod command;
mod connection;
mod file_store;
mod filter;
mod driver;
mod transaction;

pub use driver::JsonDriver;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::JsonCommand;
    use crate::connection::JsonConnection;
    use crate::file_store::{FileStore, MemoryFileStore};
    use crate::filter::{json_from_record, json_to_value, matches, parse_filter, record_from_json};
    use datasources::{
        converters::Cells, create, register, reset, CRUDRepository, CRUDRepositoryOptions, Command,
        Connection, ICRUDRepository, IReadonlyRepository, Parameter, Row, SelectStatement, Value,
        WhereText,
    };
    use serde_json::{Map, Value as JsonValue};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn crud(conn: Arc<JsonConnection>) -> CRUDRepository {
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

    #[tokio::test]
    async fn json_crud_and_transactions() {
        let store = Arc::new(MemoryFileStore::seed(
            "db.json",
            "[{\"id\":1,\"name\":\"a\"}]",
        ));
        let conn = Arc::new(JsonConnection::new("db.json", None, store.clone(), None));
        conn.open().await.unwrap();
        assert_eq!(conn.path(), "db.json");
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
        assert_eq!(removed.rows_affected, 0);
        let command = JsonCommand::new(conn.clone());
        command
            .execute("insert", Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute(
                "update",
                Some(&[
                    Parameter::new("id", 3_i64),
                    Parameter::new("name", "z"),
                    Parameter::new("$where", "{\"id\":3}"),
                ]),
            )
            .await
            .unwrap();
        command
            .execute("update", Some(&[Parameter::new("id", 3_i64)]))
            .await
            .unwrap();
        command
            .execute("delete", Some(&[Parameter::new("id", 3_i64)]))
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
        let rolled_back = repo.create(&mut rec(9, "t")).await.unwrap();
        assert_eq!(rolled_back.get("id"), Some(&Value::Int(9)));
        txn.abort().await.unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        let committed = repo.create(&mut rec(8, "c")).await.unwrap();
        assert_eq!(committed.get("id"), Some(&Value::Int(8)));
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
        let created = crud(memory.clone()).create(&mut rec(1, "a")).await.unwrap();
        assert_eq!(created.get("id"), Some(&Value::Int(1)));
        *memory.state.data.lock().unwrap() = JsonValue::Null;
        memory.collection_mut(|_| {});
        *memory.state.data.lock().unwrap() = JsonValue::Array(vec![]);
        memory.collection_mut(|_| {});
        let txn = memory.start_transaction(None).await.unwrap();
        txn.commit().await.unwrap();

        reset();
        register("json", Arc::new(JsonDriver));
        let ds = create(Some(&HashMap::from([("PROVIDER".into(), "json".into())]))).unwrap();
        assert_eq!(ds.provider(), "json");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("insert", None).await.is_err());
        assert!(command.execute_reader("", None).await.is_err());
        let bound = ds.create_command(Some(ds.create_connection().await.unwrap()));
        assert!(bound.execute("insert", None).await.is_err());
        assert!(bound.execute_reader("", None).await.is_err());
        ds.create_repository::<Cells>(None, None);
        ds.create_repository::<Cells>(Some(ds.create_connection().await.unwrap()), None);

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
            .execute_reader("", None)
            .await
            .is_err());
        assert!(JsonCommand::new(closed.clone())
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
            JsonConnection::new("", None, Arc::new(MemoryFileStore::new()), None)
                .start_transaction(None)
                .await
                .is_err()
        );

        let with_col = Arc::new(JsonConnection::new(
            "c.json",
            Some("users".into()),
            Arc::new(MemoryFileStore::seed("c.json", "{}")),
            None,
        ));
        with_col.open().await.unwrap();
        JsonCommand::new(with_col)
            .execute("update", None)
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

        json_to_value(&JsonValue::Number(
            serde_json::Number::from_f64(1.25).unwrap(),
        ));
        json_to_value(&serde_json::json!([1]));
        if let JsonValue::Number(n) = serde_json::json!(1) {
            json_to_value(&JsonValue::Number(n));
        }
    }
}
