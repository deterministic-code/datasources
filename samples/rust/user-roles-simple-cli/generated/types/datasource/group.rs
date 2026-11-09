// schema-version: 1.0
/// Type Group.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Group {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: Vec<u8>,
    pub name: String,
    pub description: String,
    pub is_enabled: bool,
}
impl Default for Group {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::DateTime::<datasources::chrono::Utc>::UNIX_EPOCH,
            updated: vec![0],
            name: String::new(),
            description: String::new(),
            is_enabled: true,
        }
    }
}
