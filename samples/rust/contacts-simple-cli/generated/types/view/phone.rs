// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View Phone.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Phone {
    pub id: i32,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub number: String,
    pub display_name: String,
    pub extention: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub phone_type_name: String,
    pub description: String,
    pub attributes: HashMap<String, String>,
    pub labels: Vec<Label>,
}
impl Default for Phone {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            contact_id: 0,
            number: String::new(),
            display_name: String::new(),
            extention: String::new(),
            is_primary: false,
            is_verified: false,
            phone_type_name: String::new(),
            description: String::new(),
            attributes: HashMap::new(),
            labels: vec![],
        }
    }
}
