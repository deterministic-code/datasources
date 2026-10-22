// schema-version: 1.0
/// Type ContactSettings.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactSettings {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub key: String,
    pub value: String,
}
impl Default for ContactSettings {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            version: vec![0],
            contact_id: 0,
            key: String::new(),
            value: String::new(),
        }
    }
}
