use crate::converters::field_converter::FieldConverter;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ProviderTypes {
    pub rename: bool,
    pub types: HashMap<String, FieldConverter>,
}
