// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View User.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct User {
    pub id: i64,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub username: String,
    pub password: String,
    pub is_enabled: bool,
    pub settings: HashMap<String, String>,
    pub roles: Vec<Role>,
    pub groups: Vec<Group>,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            username: String::new(),
            password: String::new(),
            is_enabled: true,
            settings: HashMap::new(),
            roles: vec![],
            groups: vec![],
        }
    }
}
