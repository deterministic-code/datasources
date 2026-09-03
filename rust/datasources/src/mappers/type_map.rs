use crate::mappers::type_map_field::TypeMapField;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeMap {
    pub key: String,
    pub fields: Vec<(String, TypeMapField)>,
}

impl TypeMap {
    pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self {
        Self {
            key: key.into(),
            fields,
        }
    }
}
