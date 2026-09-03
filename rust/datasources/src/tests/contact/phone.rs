use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Phone {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub number: String,
    pub label: Option<String>,
}
