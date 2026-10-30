use datasources::{FromSchema, Schema, SchemaObject, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct GeneratedShape {
    uuid: String,
    created: datasources::chrono::DateTime<datasources::chrono::Utc>,
}

impl SchemaObject for GeneratedShape {}

#[test]
fn generated_shape_tokens_compile() {
    let item = GeneratedShape {
        uuid: datasources::uuid::Uuid::new_v4().to_string(),
        created: datasources::chrono::Utc::now(),
    };
    assert!(!item.uuid.is_empty());
}

#[test]
fn from_schema_sets_string_from_uuid_and_datetime() {
    let id = Uuid::parse_str("54ef41f2-0cb4-4c72-97a7-776fd183f5ed").unwrap();
    let created = datasources::chrono::Utc::now();
    let mut row = datasources::Row::new();
    row.insert("uuid".into(), Value::Uuid(id));
    row.insert("created".into(), Value::DateTime(created));
    let schema = Schema::new(HashMap::from([
        ("uuid".into(), "uuid".into()),
        ("created".into(), "datetime".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.uuid, id.to_string());
    assert_eq!(item.created, created);
}
