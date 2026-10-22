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
impl Default for AddressBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            version: vec![0],
            contact_id: 0,
            address_type_id: 0,
            line1: String::new(),
            line2: None,
            city: String::new(),
            region: None,
            postal_code: None,
            country: None,
        }
    }
}
