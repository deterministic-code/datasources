// schema-version: 1.0
/// Type PhoneBase.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct PhoneBase {
    pub id: i32,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub phone_type_id: i32,
    pub contact_id: i32,
    pub number: String,
    pub display_name: String,
    pub extention: String,
    pub is_primary: bool,
    pub is_verified: bool,
}
impl Default for PhoneBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            phone_type_id: 0,
            contact_id: 0,
            number: String::new(),
            display_name: String::new(),
            extention: String::new(),
            is_primary: false,
            is_verified: false,
        }
    }
}
