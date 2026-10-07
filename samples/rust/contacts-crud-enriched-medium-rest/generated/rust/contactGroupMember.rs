// schema-version: 1.0
/// Type ContactGroupMember.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactGroupMember {
    pub id: i32,
    pub uuid: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub version: Vec<u8>,
    pub contact_id: String,
    pub contact_group_id: String,
}
