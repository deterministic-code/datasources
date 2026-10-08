// schema-version: 1.0
/// Type AddressBase.
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBase {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub address_type_id: i32,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
