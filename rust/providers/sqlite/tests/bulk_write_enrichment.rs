use crate::connection::SqliteConnection;
use datasources::{
    encode_rows, BatchCommandIBulkWriteStrategy, BulkCRUDRepository, Casing, ChangeCasingMapper,
    Connection, CRUDRepositoryOptions, Enrichment, EnrichmentPropertyType, Native, Row, Schema,
    Value,
};
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn num(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(n)) => *n,
        other => panic!("{other:?}"),
    }
}

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(value)) => value.clone(),
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn batch_create_stamps_generated_parent_id_onto_children() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE parents (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE children (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, label TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    let casing = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let mut child_options = CRUDRepositoryOptions::default();
    child_options.entity = "children".into();
    child_options.property_mappings = Some(casing.clone());
    child_options.ids = vec!["Id".into()];
    child_options.schema = Schema::new(HashMap::from([
        ("Id".into(), "integer".into()),
        ("ParentId".into(), "integer".into()),
        ("Label".into(), "string".into()),
    ]));
    let children = Arc::new(
        BulkCRUDRepository::with_strategy(
            connection.clone(),
            child_options,
            Arc::new(BatchCommandIBulkWriteStrategy::new()),
        )
        .unwrap(),
    );
    let mut parent_options = CRUDRepositoryOptions::default();
    parent_options.entity = "parents".into();
    parent_options.property_mappings = Some(casing);
    parent_options.ids = vec!["Id".into()];
    parent_options.schema = Schema::new(HashMap::from([
        ("Id".into(), "integer".into()),
        ("Label".into(), "string".into()),
    ]));
    let mut parents = BulkCRUDRepository::with_strategy(
        connection,
        parent_options,
        Arc::new(BatchCommandIBulkWriteStrategy::new()),
    )
    .unwrap();
    parents.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentId",
            None,
            EnrichmentPropertyType::List,
        ),
        children.clone(),
        None,
    );
    let created = parents
        .create_bulk(&mut [
            record(&[
                ("Label", Value::from("one")),
                (
                    "Children",
                    encode_rows(&[record(&[("Label", Value::from("a"))])]),
                ),
            ]),
            record(&[
                ("Label", Value::from("two")),
                (
                    "Children",
                    encode_rows(&[
                        record(&[("Label", Value::from("b"))]),
                        record(&[("Label", Value::from("c"))]),
                    ]),
                ),
            ]),
        ])
        .await
        .unwrap();
    assert_eq!(created.len(), 2);
    assert!(created.iter().all(|parent| num(parent, "Id") > 0));
    let first = num(&created[0], "Id");
    let second = num(&created[1], "Id");
    assert_ne!(first, second);
    let stored = children
        .read(None, None)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(stored.len(), 3);
    let a = stored.iter().find(|child| num(child, "ParentId") == first);
    assert_eq!(text(a.unwrap(), "Label"), "a");
    let mut rest: Vec<_> = stored
        .iter()
        .filter(|child| num(child, "ParentId") == second)
        .map(|child| text(child, "Label"))
        .collect();
    rest.sort();
    assert_eq!(rest, ["b".to_string(), "c".to_string()]);
}
