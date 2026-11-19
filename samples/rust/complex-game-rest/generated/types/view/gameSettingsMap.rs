// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View GameSettingsMap.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct GameSettingsMap {
    pub key: String,
    pub value: String,
}
