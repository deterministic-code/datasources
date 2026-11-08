// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View Game.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Game {
    pub id: i64,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub description: String,
    pub location: String,
    pub is_enabled: bool,
    pub is_deleted: bool,
    pub settings: HashMap<String, String>,
    pub categories: Vec<Category>,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            name: String::new(),
            description: String::new(),
            location: String::new(),
            is_enabled: true,
            is_deleted: false,
            settings: HashMap::new(),
            categories: vec![],
        }
    }
}
