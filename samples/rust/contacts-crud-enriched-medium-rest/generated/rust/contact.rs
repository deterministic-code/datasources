// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View Contact.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Contact {
    pub id: i32,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub contact_source_name: String,
    pub description: Option<String>,
    pub addresses: Vec<Address>,
    pub phones: Vec<Phone>,
    pub settings: HashMap<String, String>,
    pub prefs: HashMap<String, LocalePreferences>,
}
impl Default for Contact {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            first_name: String::new(),
            last_name: String::new(),
            email: None,
            notes: None,
            contact_source_name: String::new(),
            description: None,
            addresses: vec![],
            phones: vec![],
            settings: HashMap::new(),
            prefs: HashMap::new(),
        }
    }
}
