use datasources::*;
use datasources_memory::MemoryDriver;
use futures_util::TryStreamExt;
use std::sync::Arc;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn mapped_crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    mapped_crud_schema(connection, entity, Schema::default())
}

fn mapped_crud_schema(
    connection: Arc<dyn Connection>,
    entity: &str,
    schema: Schema,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.schema = schema;
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

fn marks() -> Enrichment {
    Enrichment::new("Marks", "PlayId", "PlayId", None, EnrichmentPropertyType::List)
        .with_keys(&[("PlayId", "PlayId"), ("UserId", "UserId")])
}

fn settings() -> Enrichment {
    Enrichment::new(
        "Settings",
        "PlayId",
        "PlayId",
        Some("Value".into()),
        EnrichmentPropertyType::Dictionary,
    )
    .with_dictionary_key("Name")
    .with_keys(&[("PlayId", "PlayId"), ("UserId", "UserId")])
}

fn setting_schema() -> Schema {
    Schema::new(
        [
            (
                "Id",
                SchemaColumn {
                    is_id: true,
                    ..SchemaColumn::new("integer")
                },
            ),
            ("PlayId", SchemaColumn::new("integer")),
            ("UserId", SchemaColumn::new("integer")),
            ("Name", SchemaColumn::new("string")),
            ("Value", SchemaColumn::new("string")),
        ]
        .into_iter()
        .map(|(name, column)| (name.to_string(), column))
        .collect(),
    )
}

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(value)) => value.clone(),
        _ => panic!("{key}"),
    }
}

fn num(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(id)) => *id,
        _ => panic!("{key}"),
    }
}

async fn open() -> Arc<dyn Connection> {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    connection
}

async fn seed_marks() -> (CRUDRepository, CRUDRepository) {
    let seats_connection = open().await;
    let events_connection = open().await;
    let seats = mapped_crud(seats_connection.clone(), "Seat");
    let marks_repo = mapped_crud(events_connection.clone(), "Mark");
    for (user_id, label) in [(7, "mine"), (8, "theirs"), (7, "also")] {
        events_connection
            .create_command()
            .execute(
                "insert",
                Some(&[
                    Parameter::new("play_id", Value::Int(1)),
                    Parameter::new("user_id", Value::Int(user_id)),
                    Parameter::new("label", Value::from(label)),
                ]),
            )
            .await
            .unwrap();
    }
    for user_id in [7, 8] {
        seats_connection
            .create_command()
            .execute(
                "insert",
                Some(&[
                    Parameter::new("play_id", Value::Int(1)),
                    Parameter::new("user_id", Value::Int(user_id)),
                ]),
            )
            .await
            .unwrap();
    }
    (seats, marks_repo)
}

#[tokio::test]
async fn reads_only_the_rows_that_match_every_column() {
    let (mut seats, marks_repo) = seed_marks().await;
    seats.add_enriched(marks(), Arc::new(marks_repo), None);
    let rows: Vec<Row> = seats.read(None, None).try_collect().await.unwrap();
    let mine = rows.iter().find(|row| num(row, "UserId") == 7).unwrap();
    let theirs = rows.iter().find(|row| num(row, "UserId") == 8).unwrap();
    let mut mine_labels: Vec<_> = mine
        .related::<Row>("Marks")
        .iter()
        .map(|mark| text(mark, "Label"))
        .collect();
    mine_labels.sort();
    assert_eq!(mine_labels, ["also", "mine"]);
    assert_eq!(
        theirs
            .related::<Row>("Marks")
            .iter()
            .map(|mark| text(mark, "Label"))
            .collect::<Vec<_>>(),
        ["theirs"]
    );
}

#[tokio::test]
async fn hashset_reads_only_the_rows_that_match_every_column() {
    let (mut seats, marks_repo) = seed_marks().await;
    seats.add_enriched(
        marks(),
        Arc::new(marks_repo),
        Some(Arc::new(HashsetOptimizedReadonlyEnrichmentStrategy)),
    );
    let rows: Vec<Row> = seats.read(None, None).try_collect().await.unwrap();
    let mine = rows.iter().find(|row| num(row, "UserId") == 7).unwrap();
    let theirs = rows.iter().find(|row| num(row, "UserId") == 8).unwrap();
    let mut mine_labels: Vec<_> = mine
        .related::<Row>("Marks")
        .iter()
        .map(|mark| text(mark, "Label"))
        .collect();
    mine_labels.sort();
    assert_eq!(mine_labels, ["also", "mine"]);
    assert_eq!(
        theirs
            .related::<Row>("Marks")
            .iter()
            .map(|mark| text(mark, "Label"))
            .collect::<Vec<_>>(),
        ["theirs"]
    );
}

#[tokio::test]
async fn create_stamps_every_key_and_read_does_not_mix_a_shared_first_key() {
    let connection = open().await;
    let mut seats = mapped_crud(connection.clone(), "Seat");
    let marks_repo = Arc::new(mapped_crud(connection, "Mark"));
    seats.add_enriched(marks(), marks_repo.clone(), None);
    seats
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(7)),
            (
                "Marks",
                encode_rows(&[record(&[("Label", Value::from("mine"))])]),
            ),
        ]))
        .await
        .unwrap();
    seats
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(8)),
            (
                "Marks",
                encode_rows(&[record(&[("Label", Value::from("theirs"))])]),
            ),
        ]))
        .await
        .unwrap();
    let stored: Vec<Row> = marks_repo.read(None, None).try_collect().await.unwrap();
    assert_eq!(stored.len(), 2);
    let mine = stored
        .iter()
        .find(|row| text(row, "Label") == "mine")
        .unwrap();
    let theirs = stored
        .iter()
        .find(|row| text(row, "Label") == "theirs")
        .unwrap();
    assert_eq!(num(mine, "PlayId"), 1);
    assert_eq!(num(mine, "UserId"), 7);
    assert_eq!(num(theirs, "PlayId"), 1);
    assert_eq!(num(theirs, "UserId"), 8);
    let rows: Vec<Row> = seats.read(None, None).try_collect().await.unwrap();
    assert_eq!(
        text(
            &rows
                .iter()
                .find(|row| num(row, "UserId") == 7)
                .unwrap()
                .related::<Row>("Marks")[0],
            "Label"
        ),
        "mine"
    );
    assert_eq!(
        text(
            &rows
                .iter()
                .find(|row| num(row, "UserId") == 8)
                .unwrap()
                .related::<Row>("Marks")[0],
            "Label"
        ),
        "theirs"
    );
}

#[tokio::test]
async fn create_stamps_a_dictionary_on_every_key() {
    let connection = open().await;
    let mut seats = mapped_crud(connection.clone(), "Seat");
    let settings_repo = Arc::new(mapped_crud_schema(connection, "Setting", setting_schema()));
    seats.add_enriched(settings(), settings_repo.clone(), None);
    seats
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(7)),
            (
                "Settings",
                encode_rows(&[record(&[
                    ("Name", Value::from("color")),
                    ("Value", Value::from("blue")),
                ])]),
            ),
        ]))
        .await
        .unwrap();
    seats
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(8)),
            (
                "Settings",
                encode_rows(&[record(&[
                    ("Name", Value::from("color")),
                    ("Value", Value::from("red")),
                ])]),
            ),
        ]))
        .await
        .unwrap();
    let stored: Vec<Row> = settings_repo.read(None, None).try_collect().await.unwrap();
    assert_eq!(stored.len(), 2);
    assert!(stored.iter().any(|row| {
        num(row, "PlayId") == 1
            && num(row, "UserId") == 7
            && text(row, "Name") == "color"
            && text(row, "Value") == "blue"
    }));
    assert!(stored.iter().any(|row| {
        num(row, "PlayId") == 1
            && num(row, "UserId") == 8
            && text(row, "Name") == "color"
            && text(row, "Value") == "red"
    }));
    let rows: Vec<Row> = seats.read(None, None).try_collect().await.unwrap();
    let seven = rows.iter().find(|row| num(row, "UserId") == 7).unwrap();
    let eight = rows.iter().find(|row| num(row, "UserId") == 8).unwrap();
    assert!(seven["Settings"].wire_text().contains("Value=blue"));
    assert!(!seven["Settings"].wire_text().contains("Value=red"));
    assert!(eight["Settings"].wire_text().contains("Value=red"));
    assert!(!eight["Settings"].wire_text().contains("Value=blue"));
}
