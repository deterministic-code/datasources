use crate::mappers::type_map_group::TypeMapGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeMapField {
    Column(String),
    Group(TypeMapGroup),
    Dictionary { key: String, value: String },
}

impl TypeMapField {
    pub fn column(source: impl Into<String>) -> Self {
        Self::Column(source.into())
    }

    pub fn dictionary(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Dictionary {
            key: key.into(),
            value: value.into(),
        }
    }
}
