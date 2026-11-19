// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View ContactPreferencesMap.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactPreferencesMap {
    pub key: String,
    pub value: LocalePreference,
}
