// schema-version: 1.0
/// Type ContactGroupsBase.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactGroupsBase {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
}
impl Default for ContactGroupsBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            version: vec![0],
            name: String::new(),
            description: None,
        }
    }
}
