// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View CardLabel.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct CardLabel {
    pub key: String,
    pub value: String,
}
