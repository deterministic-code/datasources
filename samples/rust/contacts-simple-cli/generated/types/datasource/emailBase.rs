// schema-version: 1.0
/// Type EmailBase.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct EmailBase {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub email_type_id: i32,
    pub contact_id: i32,
    pub address: String,
    pub display_name: String,
    pub is_primary: bool,
}
impl Default for EmailBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            email_type_id: 0,
            contact_id: 0,
            address: String::new(),
            display_name: String::new(),
            is_primary: false,
        }
    }
}
