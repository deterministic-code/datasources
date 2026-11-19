// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View CategoryTree.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct CategoryTree {
    pub id: i32,
    pub parent_category_id: Option<i32>,
    pub name: String,
    pub children: Vec<CategoryTree>,
}
