// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View LocalePreferences.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct LocalePreferences {
    pub locale: String,
    pub timezone: String,
}
