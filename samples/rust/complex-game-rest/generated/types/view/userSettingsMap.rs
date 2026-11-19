// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View UserSettingsMap.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct UserSettingsMap {
    pub key: String,
    pub value: String,
}
