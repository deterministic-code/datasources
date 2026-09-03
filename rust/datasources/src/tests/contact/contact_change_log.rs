use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ContactChangeLog {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub occurred_at: DateTime<Utc>,
    pub contact_id: i32,
    pub changed_by: String,
    pub change_summary: String,
}
