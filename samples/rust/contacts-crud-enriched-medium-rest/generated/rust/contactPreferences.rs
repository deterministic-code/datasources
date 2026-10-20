// schema-version: 1.0
/// Type ContactPreferences.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactPreferences {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub key: String,
    pub locale: String,
    pub timezone: String,
}
