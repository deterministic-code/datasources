// schema-version: 1.0
/// Type UserRole.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct UserRole {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: Vec<u8>,
    pub user_id: i32,
    pub role_id: i32,
}
impl Default for UserRole {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::DateTime::<datasources::chrono::Utc>::UNIX_EPOCH,
            updated: vec![0],
            user_id: 0,
            role_id: 0,
        }
    }
}
