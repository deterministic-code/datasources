use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ContactGroupBase {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
}
