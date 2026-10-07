// schema-version: 1.0
/// Type Base.
#[derive(Clone, Debug, PartialEq)]
pub struct Base {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
}
