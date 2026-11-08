// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View ContactCard.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactCard {
    pub display_name: String,
}
