use chrono::{DateTime, Utc};

pub struct LegacyMigrationResult {
    pub migrated_count: i32,
    pub skipped_count: i32,
    pub completed_at: Option<DateTime<Utc>>,
}
