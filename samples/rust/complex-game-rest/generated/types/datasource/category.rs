// schema-version: 1.0
/// Type Category.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Category {
    pub id: i64,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub category_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub is_enabled: bool,
    pub is_deleted: bool,
}
impl Default for Category {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            category_id: None,
            name: String::new(),
            description: String::new(),
            is_enabled: true,
            is_deleted: false,
        }
    }
}
