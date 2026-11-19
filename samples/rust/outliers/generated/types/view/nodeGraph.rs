// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View NodeGraph.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct NodeGraph {
    pub id: i32,
    pub value: String,
    pub edges: Vec<NodeGraph>,
}
