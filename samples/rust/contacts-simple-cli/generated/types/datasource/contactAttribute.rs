// schema-version: 1.0
/// Type ContactAttribute.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactAttribute {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub key: String,
    pub value: String,
}
impl Default for ContactAttribute {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            contact_id: 0,
            key: String::new(),
            value: String::new(),
        }
    }
}
