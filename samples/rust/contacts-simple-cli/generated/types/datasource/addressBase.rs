// schema-version: 1.0
/// Type AddressBase.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct AddressBase {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub address_type_id: i32,
    pub display_name: String,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub neighborhood: String,
    pub district: String,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country_code: String,
    pub formatted_address: String,
    pub latitude: Option<i32>,
    pub longitude: Option<i32>,
    pub is_primary: bool,
}
impl Default for AddressBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            contact_id: 0,
            address_type_id: 0,
            display_name: String::new(),
            line_1: String::new(),
            line_2: String::new(),
            line_3: String::new(),
            neighborhood: String::new(),
            district: String::new(),
            city: String::new(),
            state_province: String::new(),
            postal_code: String::new(),
            country_code: String::new(),
            formatted_address: String::new(),
            latitude: None,
            longitude: None,
            is_primary: false,
        }
    }
}
