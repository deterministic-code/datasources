// schema-version: 1.0
/// Type WardNode.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct WardNode {
    pub id: i32,
    pub parent_region_id: i32,
    pub parent_ward_id: i32,
    pub name: String,
}
