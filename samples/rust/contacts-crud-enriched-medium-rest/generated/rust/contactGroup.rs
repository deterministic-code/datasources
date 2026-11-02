// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View ContactGroup.
#[derive(Clone, Debug, PartialEq, Default, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactGroup {
    pub base: ContactGroupsBase,
    pub members: Vec<Contact>,
}
