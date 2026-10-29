use datasources::SchemaObject;

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
