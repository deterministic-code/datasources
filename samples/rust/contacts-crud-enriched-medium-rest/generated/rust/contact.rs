// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View Contact.
#[derive(Clone, Debug, PartialEq)]
pub struct Contact {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_source_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub contact_source_name: String,
    pub addresses: Vec<crate::types::generated::views::Address>,
    pub phones: Vec<crate::types::generated::views::Phone>,
}
