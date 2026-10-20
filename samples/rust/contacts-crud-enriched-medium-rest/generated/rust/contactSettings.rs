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
