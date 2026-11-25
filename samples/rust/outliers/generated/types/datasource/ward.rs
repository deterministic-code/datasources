// schema-version: 1.0
/// Type Ward.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Ward {
    pub region_id: i32,
    pub ward_id: i32,
    pub name: String,
}
