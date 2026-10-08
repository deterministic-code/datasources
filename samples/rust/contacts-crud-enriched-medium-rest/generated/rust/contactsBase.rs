// schema-version: 1.0
/// Type ContactsBase.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactsBase {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_source_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
}
