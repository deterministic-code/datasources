use crate::{
    BatchCommandIBulkWriteStrategy, BulkCRUDRepository, CRUDRepositoryOptions, MockCommand,
    MockConnection, RowTypeConverter, SingleCommandIBulkWriteStrategy,
    StoredProcedureBulkWriteStrategy, Value,
};
use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

fn row(pairs: &[(&str, Value)]) -> crate::Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn parents(strategy: Arc<dyn crate::BulkWriteStrategy>) -> (BulkCRUDRepository, Arc<MockCommand>) {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "parents".into();
    options.row_converter = Some(RowTypeConverter::new(vec!["id".into(), "label".into()]));
    let repo = BulkCRUDRepository::with_strategy(connection, options, strategy).unwrap();
    (repo, command)
}

#[tokio::test]
async fn single_command_i_bulk_write_strategy_writes_one_statement_per_row() {
    let (repo, command) = parents(Arc::new(SingleCommandIBulkWriteStrategy));
    *command.response.lock().unwrap() = crate::Response::new(Value::Int(9), 1, None);
    let mut created = [
        row(&[("label", Value::from("a"))]),
        row(&[("label", Value::from("b"))]),
    ];
    repo.create_bulk(&mut created).await.unwrap();
    repo.update_bulk(
        &[
            row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
            row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
        ],
        None,
    )
    .await
    .unwrap();
    repo.delete_bulk(
        &[
            row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
            row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
        ],
        None,
    )
    .await
    .unwrap();
    let logs = command.execute_logs.lock().unwrap();
    assert_eq!(logs.len(), 6);
    assert_eq!(logs[0].0, "INSERT INTO parents (label) VALUES ($label)");
    assert_eq!(logs[1].0, "INSERT INTO parents (label) VALUES ($label)");
    assert_eq!(
        logs[2].0,
        "UPDATE parents SET label = $label WHERE id = $id"
    );
    assert_eq!(
        logs[3].0,
        "UPDATE parents SET label = $label WHERE id = $id"
    );
    assert_eq!(logs[4].0, "DELETE FROM parents WHERE id = $id");
    assert_eq!(logs[5].0, "DELETE FROM parents WHERE id = $id");
}

#[tokio::test]
async fn batch_command_i_bulk_write_strategy_joins_statements() {
    let (repo, command) = parents(Arc::new(BatchCommandIBulkWriteStrategy));
    let mut created = [
        row(&[("label", Value::from("a"))]),
        row(&[("label", Value::from("b"))]),
    ];
    repo.create_bulk(&mut created).await.unwrap();
    {
        let logs = command.execute_logs.lock().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(
            logs[0].0,
            "INSERT INTO parents (label) VALUES ($label__0); INSERT INTO parents (label) VALUES ($label__1)"
        );
        assert_eq!(logs[0].1[0].name, "label__0");
        assert_eq!(logs[0].1[0].value, Value::from("a"));
        assert_eq!(logs[0].1[1].name, "label__1");
        assert_eq!(logs[0].1[1].value, Value::from("b"));
    }
    repo.update_bulk(
        &[
            row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
            row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
        ],
        None,
    )
    .await
    .unwrap();
    repo.delete_bulk(
        &[row(&[("id", Value::Int(1))]), row(&[("id", Value::Int(2))])],
        None,
    )
    .await
    .unwrap();
    let logs = command.execute_logs.lock().unwrap();
    assert_eq!(
        logs[1].0,
        "UPDATE parents SET label = $label__0 WHERE id = $id__0; UPDATE parents SET label = $label__1 WHERE id = $id__1"
    );
    assert_eq!(
        logs[1]
            .1
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["label__0", "id__0", "label__1", "id__1"]
    );
    assert_eq!(
        logs[2].0,
        "DELETE FROM parents WHERE id = $id__0; DELETE FROM parents WHERE id = $id__1"
    );
    assert_eq!(
        logs[2]
            .1
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["id__0", "id__1"]
    );
}

#[tokio::test]
async fn stored_procedure_bulk_write_strategy_sends_one_json_call() {
    let (repo, command) = parents(Arc::new(StoredProcedureBulkWriteStrategy::new(
        "bulk_create_parent",
        "bulk_update_parent",
        "bulk_delete_parent",
    )));
    let mut created = [
        row(&[("label", Value::from("a"))]),
        row(&[("label", Value::from("b"))]),
    ];
    repo.create_bulk(&mut created).await.unwrap();
    repo.update_bulk(
        &[
            row(&[("id", Value::Int(1)), ("label", Value::from("a"))]),
            row(&[("id", Value::Int(2)), ("label", Value::from("b"))]),
        ],
        None,
    )
    .await
    .unwrap();
    repo.delete_bulk(
        &[
            row(&[("id", Value::Int(1)), ("label", Value::from(""))]),
            row(&[("id", Value::Int(2)), ("label", Value::from(""))]),
        ],
        None,
    )
    .await
    .unwrap();
    let logs = command.execute_logs.lock().unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].0, "CALL bulk_create_parent($payload)");
    assert_eq!(logs[0].1[0].name, "payload");
    assert_eq!(
        logs[0].1[0].value,
        Value::from(r#"[{"id":null,"label":"a"},{"id":null,"label":"b"}]"#)
    );
    assert_eq!(logs[1].0, "CALL bulk_update_parent($payload)");
    assert_eq!(
        logs[1].1[0].value,
        Value::from(r#"[{"id":1,"label":"a"},{"id":2,"label":"b"}]"#)
    );
    assert_eq!(logs[2].0, "CALL bulk_delete_parent($payload)");
    assert_eq!(
        logs[2].1[0].value,
        Value::from(r#"[{"id":1,"label":""},{"id":2,"label":""}]"#)
    );
}

#[tokio::test]
async fn payload_encodes_scalars() {
    let command = Arc::new(MockCommand::new());
    let connection = MockConnection::with_command_provider(command.clone(), "memory");
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "parents".into();
    options.ids = Vec::new();
    options.row_converter = Some(RowTypeConverter::new(vec![
        "amount".into(),
        "blob".into(),
        "count".into(),
        "created".into(),
        "flag".into(),
        "label".into(),
        "price".into(),
        "token".into(),
    ]));
    let repo = BulkCRUDRepository::with_strategy(
        connection,
        options,
        Arc::new(StoredProcedureBulkWriteStrategy::new(
            "create_scalar",
            "update_scalar",
            "delete_scalar",
        )),
    )
    .unwrap();
    let created = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
    let mut items = [row(&[
        ("amount", Value::Float(1.25)),
        ("blob", Value::Binary(vec![1, 2, 3])),
        ("count", Value::Int(2)),
        ("created", Value::DateTime(created)),
        ("flag", Value::Bool(true)),
        ("price", Value::Decimal(Decimal::new(105, 1))),
        (
            "token",
            Value::Uuid(Uuid::parse_str("3f2504e0-4f89-11d3-9a0c-0305e82c3301").unwrap()),
        ),
    ])];
    repo.create_bulk(&mut items).await.unwrap();
    let logs = command.execute_logs.lock().unwrap();
    assert_eq!(
        logs[0].1[0].value,
        Value::from(
            r#"[{"amount":1.25,"blob":"AQID","count":2,"created":"2024-06-01T00:00:00.0000000Z","flag":true,"label":null,"price":"10.5","token":"3f2504e0-4f89-11d3-9a0c-0305e82c3301"}]"#
        )
    );
}

#[tokio::test]
async fn empty_list_does_not_execute() {
    for strategy in [
        Arc::new(SingleCommandIBulkWriteStrategy) as Arc<dyn crate::BulkWriteStrategy>,
        Arc::new(BatchCommandIBulkWriteStrategy),
        Arc::new(StoredProcedureBulkWriteStrategy::new(
            "create_parent",
            "update_parent",
            "delete_parent",
        )),
    ] {
        let (repo, command) = parents(strategy);
        let created = repo.create_bulk(&mut []).await.unwrap();
        let updated = repo.update_bulk(&[], None).await.unwrap();
        let deleted = repo.delete_bulk(&[], None).await.unwrap();
        assert!(command.execute_logs.lock().unwrap().is_empty());
        assert_eq!(created.rows_affected, 0);
        assert_eq!(updated.rows_affected, 0);
        assert_eq!(deleted.rows_affected, 0);
        assert_eq!(created.result, Value::Null);
    }
}
