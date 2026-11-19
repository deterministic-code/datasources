// schema-version: 1.0
/// Type NodeEdge.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct NodeEdge {
    pub from_node_id: i32,
    pub to_node_id: i32,
}
