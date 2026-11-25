// schema-version: 1.0
/// Type WardAlias.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct WardAlias {
    pub id: i32,
    pub parent_region_id: i32,
    pub parent_ward_id: i32,
    pub name: String,
}
