// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View ContactSettingsMap.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactSettingsMap {
    pub key: String,
    pub value: String,
}
