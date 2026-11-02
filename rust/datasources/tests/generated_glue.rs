use datasources::{encode_rows, FromSchema, Row, Schema, SchemaObject, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct LocalePref {
    locale: String,
    timezone: String,
}

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct GeneratedShape {
    uuid: String,
    created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    version: Vec<u8>,
    contact_id: String,
    line2: Option<String>,
    settings: HashMap<String, String>,
    prefs: HashMap<String, LocalePref>,
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
        settings: HashMap::new(),
        prefs: HashMap::new(),
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

#[test]
fn from_schema_sets_datetime_from_iso_text() {
    let mut row = datasources::Row::new();
    row.insert("uuid".into(), Value::from("54ef41f2-0cb4-4c72-97a7-776fd183f5ed"));
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    let schema = Schema::new(HashMap::from([
        ("uuid".into(), "uuid".into()),
        ("created".into(), "datetime".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.created.to_rfc3339_opts(datasources::chrono::SecondsFormat::Millis, true), "2026-10-30T02:04:49.339Z");
}

#[test]
fn from_schema_sets_bytes_from_base64_text() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("version".into(), Value::from("AA=="));
    let schema = Schema::new(HashMap::from([
        ("created".into(), "datetime".into()),
        ("version".into(), "binary".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.version, vec![0]);
}

#[test]
fn from_schema_sets_i32_from_empty_text() {
    #[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
    struct WithId {
        id: i32,
    }
    impl SchemaObject for WithId {}
    let mut row = datasources::Row::new();
    row.insert("id".into(), Value::from(""));
    let schema = Schema::new(HashMap::from([("id".into(), "integer".into())]));
    let item = WithId::from_schema(row, &schema).unwrap();
    assert_eq!(item.id, 0);
}

#[test]
fn from_schema_sets_string_from_int() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("contact_id".into(), Value::Int(1));
    let schema = Schema::new(HashMap::from([
        ("created".into(), "datetime".into()),
        ("contact_id".into(), "integer".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.contact_id, "1");
}

#[test]
fn from_schema_sets_option_string_from_text() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("line2".into(), Value::from("flat 2"));
    let schema = Schema::new(HashMap::from([
        ("created".into(), "datetime".into()),
        ("line2".into(), "string".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.line2.as_deref(), Some("flat 2"));
}

#[test]
fn from_schema_sets_option_none_from_null() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("line2".into(), Value::Null);
    let schema = Schema::new(HashMap::from([
        ("created".into(), "datetime".into()),
        ("line2".into(), "string".into()),
    ]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.line2, None);
}

fn encoded(pairs: &[(&str, Value)]) -> Value {
    let mut row = Row::new();
    for (key, value) in pairs {
        row.insert((*key).into(), value.clone());
    }
    encode_rows(&[row])
}

#[test]
fn from_schema_sets_empty_map_from_empty_text() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("settings".into(), Value::from(""));
    let schema = Schema::new(HashMap::from([("created".into(), "datetime".into())]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert!(item.settings.is_empty());
}

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct ChildShape {
    attributes: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct ParentShape {
    children: Vec<ChildShape>,
}

impl SchemaObject for ParentShape {}

#[test]
fn from_schema_sets_nested_empty_map_from_empty_text() {
    let mut row = datasources::Row::new();
    row.insert("children".into(), encoded(&[("attributes", Value::from(""))]));
    let item = ParentShape::from_schema(row, &Schema::new(HashMap::new())).unwrap();
    assert_eq!(item.children.len(), 1);
    assert!(item.children[0].attributes.is_empty());
}

#[test]
fn from_schema_sets_string_map_from_encoded_rows() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert("settings".into(), encoded(&[("key", Value::from("theme")), ("value", Value::from("dark"))]));
    let schema = Schema::new(HashMap::from([("created".into(), "datetime".into())]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(item.settings.get("theme").map(String::as_str), Some("dark"));
}

#[test]
fn from_schema_sets_struct_map_from_encoded_rows() {
    let mut row = datasources::Row::new();
    row.insert("created".into(), Value::from("2026-10-30T02:04:49.339Z"));
    row.insert(
        "prefs".into(),
        encoded(&[
            ("key", Value::from("home")),
            ("locale", Value::from("en-GB")),
            ("timezone", Value::from("Europe/London")),
        ]),
    );
    let schema = Schema::new(HashMap::from([("created".into(), "datetime".into())]));
    let item = GeneratedShape::from_schema(row, &schema).unwrap();
    let pref = item.prefs.get("home").expect("home");
    assert_eq!(pref.locale, "en-GB");
    assert_eq!(pref.timezone, "Europe/London");
}

#[test]
fn to_schema_encodes_maps() {
    let item = GeneratedShape {
        created: datasources::chrono::Utc::now(),
        settings: HashMap::from([("theme".into(), "dark".into())]),
        prefs: HashMap::from([(
            "home".into(),
            LocalePref {
                locale: "en-GB".into(),
                timezone: "Europe/London".into(),
            },
        )]),
        ..Default::default()
    };
    let schema = Schema::new(HashMap::new());
    let row = item.to_schema(&schema).unwrap();
    let back = GeneratedShape::from_schema(row, &schema).unwrap();
    assert_eq!(back.settings.get("theme").map(String::as_str), Some("dark"));
    assert_eq!(back.prefs.get("home").map(|pref| pref.locale.as_str()), Some("en-GB"));
}
