// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View BadgeSettingMap.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct BadgeSettingMap {
    pub key: String,
    pub value: String,
}
