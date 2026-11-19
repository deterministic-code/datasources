// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View RegionCategoryTree.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct RegionCategoryTree {
    pub region_id: i32,
    pub category_id: i32,
    pub parent_region_id: Option<i32>,
    pub parent_category_id: Option<i32>,
    pub name: String,
    pub children: Vec<RegionCategoryTree>,
}
