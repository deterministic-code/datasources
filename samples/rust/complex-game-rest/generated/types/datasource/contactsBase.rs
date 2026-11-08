// schema-version: 1.0
/// Type ContactsBase.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactsBase {
    pub id: i64,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_source_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub user_id: i64,
}
impl Default for ContactsBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            contact_source_id: 0,
            first_name: String::new(),
            last_name: String::new(),
            email: None,
            notes: None,
            user_id: 0,
        }
    }
}
