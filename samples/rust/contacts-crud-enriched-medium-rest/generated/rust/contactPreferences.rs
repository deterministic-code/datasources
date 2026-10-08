// schema-version: 1.0
/// Type ContactPreferences.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactPreferences {
    pub contact_id: String,
    pub key: String,
    pub locale: String,
    pub timezone: String,
}
