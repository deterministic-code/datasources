use chrono::{DateTime, Utc};

pub struct LegacyContact {
    pub key: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub imported_at: Option<DateTime<Utc>>,
}
