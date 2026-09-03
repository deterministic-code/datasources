use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ContactGroupMember {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub contact_id: i32,
    pub contact_group_id: i32,
}
