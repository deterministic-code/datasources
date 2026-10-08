// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View Contact.
#[derive(Clone, Debug, PartialEq)]
pub struct Contact {
    pub base: crate::types::generated::datasource::ContactsBase,
    pub addresses: Vec<crate::types::generated::views::Address>,
    pub phones: Vec<crate::types::generated::views::Phone>,
    pub settings: HashMap<String, String>,
    pub prefs: HashMap<String, crate::types::generated::views::LocalePreferences>,
}
