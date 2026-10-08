// schema-version: 1.0
/// Type AddressType.
#[derive(Clone, Debug, PartialEq)]
pub struct AddressType {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
}
