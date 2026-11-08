// schema-version: 1.0
/// Type LegacyContact.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct LegacyContact {
    pub key: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub imported_at: Option<datasources::chrono::DateTime<datasources::chrono::Utc>>,
}
