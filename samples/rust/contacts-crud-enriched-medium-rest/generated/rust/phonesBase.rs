// schema-version: 1.0
/// Type PhonesBase.
#[derive(Clone, Debug, PartialEq)]
pub struct PhonesBase {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: String,
    pub number: String,
    pub label: Option<String>,
}
