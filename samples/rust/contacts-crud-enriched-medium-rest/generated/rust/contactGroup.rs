// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View ContactGroup.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactGroup {
    pub base: crate::types::generated::datasource::ContactGroupsBase,
    pub members: Vec<crate::types::generated::views::Contact>,
}
