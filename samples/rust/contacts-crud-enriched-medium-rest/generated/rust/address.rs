// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View Address.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Address {
    pub id: i32,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub latitude: Option<i32>,
    pub label_note: Option<String>,
    pub floor: Option<u32>,
    pub score: Option<String>,
    pub external_id: Option<String>,
    pub is_verified: Option<bool>,
    pub verified_at: Option<datasources::chrono::DateTime<datasources::chrono::Utc>>,
    pub stamp: Option<Vec<u8>>,
    pub contact_id: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub address_type_name: String,
    pub description: Option<String>,
}
impl Default for Address {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            latitude: None,
            label_note: None,
            floor: None,
            score: None,
            external_id: None,
            is_verified: None,
            verified_at: None,
            stamp: None,
            contact_id: String::new(),
            line1: String::new(),
            line2: None,
            city: String::new(),
            region: None,
            postal_code: None,
            country: None,
            address_type_name: String::new(),
            description: None,
        }
    }
}
