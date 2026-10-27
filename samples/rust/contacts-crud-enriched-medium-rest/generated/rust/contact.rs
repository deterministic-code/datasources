// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View Contact.
#[derive(Clone, Debug, PartialEq)]
pub struct Contact {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub contact_source_name: String,
    pub description: Option<String>,
    pub addresses: Vec<crate::types::generated::views::Address>,
    pub phones: Vec<crate::types::generated::views::Phone>,
    pub settings: HashMap<String, String>,
    pub prefs: HashMap<String, crate::types::generated::views::LocalePreferences>,
}
impl Default for Contact {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
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
