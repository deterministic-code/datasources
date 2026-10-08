// schema-version: 1.0
/// Type PhoneBase.
#[derive(Clone, Debug, PartialEq)]
pub struct PhoneBase {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub phone_type_id: i32,
    pub number: String,
    pub label: Option<String>,
}
