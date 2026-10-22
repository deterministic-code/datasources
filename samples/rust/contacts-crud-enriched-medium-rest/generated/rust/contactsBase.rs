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
impl Default for ContactsBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            version: vec![0],
            contact_source_id: 0,
            first_name: String::new(),
            last_name: String::new(),
            email: None,
            notes: None,
        }
    }
}
