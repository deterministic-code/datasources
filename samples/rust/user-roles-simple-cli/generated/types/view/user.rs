// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View User.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct User {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: Vec<u8>,
    pub username: String,
    pub password: String,
    pub is_enabled: bool,
    pub roles: Vec<Role>,
    pub groups: Vec<Group>,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: vec![0],
            username: String::new(),
            password: String::new(),
            is_enabled: true,
            roles: vec![],
            groups: vec![],
        }
    }
}
