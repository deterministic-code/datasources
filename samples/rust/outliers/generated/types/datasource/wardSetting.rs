// schema-version: 1.0
/// Type WardSetting.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct WardSetting {
    pub id: i32,
    pub parent_region_id: i32,
    pub parent_ward_id: i32,
    pub name: String,
    pub value: String,
}
