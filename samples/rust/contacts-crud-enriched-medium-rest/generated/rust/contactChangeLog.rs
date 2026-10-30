// schema-version: 1.0
/// Type ContactChangeLog.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactChangeLog {
    pub id: i32,
    pub occurred_at: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub contact_id: i32,
    pub changed_by: String,
    pub change_summary: String,
}
