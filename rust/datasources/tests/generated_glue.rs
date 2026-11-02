use datasources::{MetaDataRowTypeConverter, Row, SchemaObject, TypeConverter, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct GeneratedShape {
    uuid: String,
    created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    version: Vec<u8>,
    contact_id: String,
    line2: Option<String>,
}

impl SchemaObject for GeneratedShape {}

#[test]
fn generated_shape_tokens_compile() {
    let item = GeneratedShape {
        uuid: datasources::uuid::Uuid::new_v4().to_string(),
        created: datasources::chrono::Utc::now(),
        version: vec![0],
        contact_id: "1".into(),
        line2: None,
    };
    assert!(!item.uuid.is_empty());
}

#[test]
fn object_converter_assigns_matching_cells() {
    let created = datasources::chrono::Utc::now();
    let mut row = Row::new();
    row.insert("uuid".into(), Value::from("54ef41f2-0cb4-4c72-97a7-776fd183f5ed"));
    row.insert("created".into(), Value::DateTime(created));
    row.insert("version".into(), Value::Binary(vec![0]));
    row.insert("contact_id".into(), Value::from("1"));
    row.insert("line2".into(), Value::from("flat 2"));
    let item = MetaDataRowTypeConverter::<GeneratedShape>::new()
        .convert(&row)
        .unwrap();
    assert_eq!(item.uuid, "54ef41f2-0cb4-4c72-97a7-776fd183f5ed");
    assert_eq!(item.created, created);
    assert_eq!(item.version, vec![0]);
    assert_eq!(item.contact_id, "1");
    assert_eq!(item.line2.as_deref(), Some("flat 2"));
}

#[test]
fn object_converter_uses_default_for_null() {
    let created = datasources::chrono::Utc::now();
    let mut row = Row::new();
    row.insert("uuid".into(), Value::from("54ef41f2-0cb4-4c72-97a7-776fd183f5ed"));
    row.insert("created".into(), Value::DateTime(created));
    row.insert("version".into(), Value::Binary(vec![0]));
    row.insert("contact_id".into(), Value::from("1"));
    row.insert("line2".into(), Value::Null);
    let item = MetaDataRowTypeConverter::<GeneratedShape>::new()
        .convert(&row)
        .unwrap();
    assert_eq!(item.line2, None);
}

#[test]
fn object_converter_leaves_children_at_default() {
    #[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
    struct WithMap {
        settings: HashMap<String, String>,
    }
    impl SchemaObject for WithMap {}
    let mut row = Row::new();
    row.insert("settings".into(), Value::from("key=theme,value=dark"));
    let item = MetaDataRowTypeConverter::<WithMap>::new()
        .convert(&row)
        .unwrap();
    assert!(item.settings.is_empty());
}

#[test]
fn object_converter_writes_columns_only() {
    let id = Uuid::parse_str("54ef41f2-0cb4-4c72-97a7-776fd183f5ed").unwrap();
    let created = datasources::chrono::Utc::now();
    let item = GeneratedShape {
        uuid: id.to_string(),
        created,
        version: vec![0],
        contact_id: "1".into(),
        line2: Some("flat 2".into()),
    };
    let row = MetaDataRowTypeConverter::<GeneratedShape>::new()
        .reverse_convert(&item)
        .unwrap();
    assert_eq!(row.get("uuid"), Some(&Value::from(id.to_string())));
    assert_eq!(row.get("created"), Some(&Value::DateTime(created)));
    assert_eq!(row.get("version"), Some(&Value::Binary(vec![0])));
    assert_eq!(row.get("settings"), None);
}
