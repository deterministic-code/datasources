// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View BadgeView.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct BadgeView {
    pub badge_id: datasources::uuid::Uuid,
    pub code: String,
    pub name: String,
    pub aliases: Vec<BadgeAlias>,
    pub settings: HashMap<String, String>,
    pub nodes: Vec<BadgeNode>,
}
