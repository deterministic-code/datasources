// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View WardView.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct WardView {
    pub region_id: i32,
    pub ward_id: i32,
    pub name: String,
    pub aliases: Vec<WardAlias>,
    pub settings: HashMap<String, String>,
    pub nodes: Vec<WardNode>,
}
