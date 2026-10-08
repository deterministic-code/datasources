// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
use std::collections::HashMap;
/// View ContactCard.
#[derive(Clone, Debug, PartialEq)]
pub struct ContactCard {
    pub display_name: String,
    pub labels: HashMap<String, String>,
}
