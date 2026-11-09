// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View LocalePreference.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct LocalePreference {
    pub locale: String,
    pub timezone: String,
}
