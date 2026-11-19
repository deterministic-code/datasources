// schema-version: 1.0
/// Type RegionCategory.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct RegionCategory {
    pub region_id: i32,
    pub category_id: i32,
    pub parent_region_id: Option<i32>,
    pub parent_category_id: Option<i32>,
    pub name: String,
}
