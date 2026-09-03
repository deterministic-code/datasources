use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::Contact;

pub struct ContactGroup {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<Contact>,
}
