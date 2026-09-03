use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{Address, Phone};

pub struct Contact {
    pub id: i32,
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub version: Vec<u8>,
    pub contact_source_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub contact_source_name: String,
    pub contact_type_name: String,
    pub addresses: Vec<Address>,
    pub phones: Vec<Phone>,
}
