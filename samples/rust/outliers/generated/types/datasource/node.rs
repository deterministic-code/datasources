// schema-version: 1.0
/// Type Node.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Node {
    pub id: i32,
    pub value: String,
}
