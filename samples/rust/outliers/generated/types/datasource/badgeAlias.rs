// schema-version: 1.0
/// Type BadgeAlias.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct BadgeAlias {
    pub id: i32,
    pub parent_badge_id: datasources::uuid::Uuid,
    pub parent_code: String,
    pub name: String,
}
