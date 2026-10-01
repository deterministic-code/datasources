use datasources::{
    default, ConvertSchema, DictionaryMapper, Driver, Mapper, MetaDataRowTypeConverter, Parameter,
    Row, RowTypeConverter, Value,
};
use datasources_sql_specs::apply_schema;
use futures_util::TryStreamExt;
use std::collections::HashMap;
use uuid::Uuid;

struct User;

impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Id", "INTEGER"),
            ("Uuid", "TEXT"),
            ("Username", "TEXT"),
            ("IsEnabled", "INTEGER"),
            ("Created", "TEXT"),
        ]
    }
}

#[tokio::test]
async fn maps_and_converts_users_through_the_read_pipeline() {
    let _guard = super::env_lock();
    let ada = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let grace = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
    let lin = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();

    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    apply_schema(
        ds.create_connection().await.unwrap().as_ref(),
        "pipeline-users",
    )
    .await
    .unwrap();
    command
        .execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            Some(&[
                Parameter::new("id", 1_i64),
                Parameter::new("uuid", ada.to_string()),
                Parameter::new("name", "ada"),
                Parameter::new("is_enabled", 1_i64),
                Parameter::new("created", "2001-01-01T00:00:00.000Z"),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            Some(&[
                Parameter::new("id", 2_i64),
                Parameter::new("uuid", grace.to_string()),
                Parameter::new("name", "grace"),
                Parameter::new("is_enabled", 1_i64),
                Parameter::new("created", "2002-02-02T00:00:00.000Z"),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "INSERT INTO users (id, uuid, name, is_enabled, created) VALUES ($id, $uuid, $name, $is_enabled, $created)",
            Some(&[
                Parameter::new("id", 3_i64),
                Parameter::new("uuid", lin.to_string()),
                Parameter::new("name", "lin"),
                Parameter::new("is_enabled", 0_i64),
                Parameter::new("created", "2003-03-03T00:00:00.000Z"),
            ]),
        )
        .await
        .unwrap();

    let property_to_column_mapper = DictionaryMapper::new([
        ("Id".into(), "id".into()),
        ("Uuid".into(), "uuid".into()),
        ("Username".into(), "name".into()),
        ("IsEnabled".into(), "is_enabled".into()),
        ("Created".into(), "created".into()),
    ]);

    let reader = command
        .execute_reader("SELECT id, uuid, name, is_enabled, created FROM users", None)
        .await
        .unwrap();
    let unmapped: Vec<Row> = reader.read().try_collect().await.unwrap();

    let column_mapped_rows: Vec<Row> = unmapped
        .iter()
        .map(|row| {
            row.iter()
                .map(|(key, value)| {
                    (
                        Mapper::reverse_map(&property_to_column_mapper, key.clone()),
                        value.clone(),
                    )
                })
                .collect()
        })
        .collect();

    let columns = HashMap::from([
        ("Uuid".into(), "uuid".into()),
        ("IsEnabled".into(), "bool".into()),
        ("Created".into(), "datetime".into()),
    ]);
    let row_converter = RowTypeConverter::from(|| crate::SqliteDriver.row_type_converters(), &columns);
    let converted_rows: Vec<Row> = column_mapped_rows
        .iter()
        .map(|row| row_converter.convert(row).unwrap())
        .collect();

    let object_converter = MetaDataRowTypeConverter::of::<User>();
    let final_entities: Vec<Row> = converted_rows
        .iter()
        .map(|row| object_converter.convert(row).unwrap())
        .collect();

    assert_eq!(unmapped.len(), 3);
    assert_eq!(unmapped[0].get("id"), Some(&Value::Int(1)));
    assert_eq!(unmapped[0].get("uuid"), Some(&Value::from(ada.to_string())));
    assert_eq!(unmapped[0].get("name"), Some(&Value::from("ada")));
    assert!(matches!(
        unmapped[0].get("is_enabled"),
        Some(Value::Bool(_)) | Some(Value::Int(_))
    ));
    assert!(matches!(unmapped[0].get("created"), Some(Value::Text(_))));

    assert_eq!(column_mapped_rows[0].get("Id"), Some(&Value::Int(1)));
    assert_eq!(
        column_mapped_rows[0].get("Username"),
        Some(&Value::from("ada"))
    );
    assert!(matches!(converted_rows[0].get("Uuid"), Some(Value::Uuid(_))));
    assert_eq!(converted_rows[0].get("IsEnabled"), Some(&Value::Bool(true)));
    assert!(matches!(
        converted_rows[0].get("Created"),
        Some(Value::DateTime(_))
    ));

    assert_eq!(final_entities[0].get("Id"), Some(&Value::Int(1)));
    assert!(matches!(final_entities[0].get("Uuid"), Some(Value::Uuid(_))));
    assert_eq!(
        final_entities[0].get("Username"),
        Some(&Value::from("ada"))
    );
    assert_eq!(
        final_entities[1].get("Username"),
        Some(&Value::from("grace"))
    );
    assert_eq!(
        final_entities[2].get("IsEnabled"),
        Some(&Value::Bool(false))
    );
}
