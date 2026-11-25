// schema-version: 1.0
/// Type Badge.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Badge {
    pub badge_id: datasources::uuid::Uuid,
    pub code: String,
    pub name: String,
}
