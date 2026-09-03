use crate::mappers::type_map_field::TypeMapField;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeMapGroup {
    pub key: String,
    pub fields: Vec<(String, TypeMapField)>,
}

impl TypeMapGroup {
    pub fn new(key: impl Into<String>, fields: Vec<(String, TypeMapField)>) -> Self {
        Self {
            key: key.into(),
            fields,
        }
    }
}
