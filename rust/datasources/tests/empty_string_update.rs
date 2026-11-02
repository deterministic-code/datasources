use datasources::*;
use datasources_memory::MemoryDriver;
use std::collections::HashMap;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

#[tokio::test]
async fn update_keeps_explicit_empty_string_on_nullable() {
    let connection = MemoryDriver.connection(None);
    let mut options = CRUDRepositoryOptions::default();
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn::new("integer")),
        (
            "note".into(),
            SchemaColumn {
                type_name: "string".into(),
                nullable: true,
                has_default: false,
                is_id: false,
            },
        ),
    ]));
    options.update_properties = Some(vec!["note".into()]);
    let repo = CRUDRepository::<Row>::new(connection, options).unwrap();
    repo.create(record(&[("id", Value::Int(1)), ("note", Value::from("side"))]))
        .await
        .expect("create");
    repo.update(record(&[("id", Value::Int(1)), ("note", Value::from(""))]))
        .await
        .expect("update");
    assert_eq!(
        repo.read_single(&()).await.unwrap().unwrap()["note"],
        Value::from("")
    );
}
