use crate::mappers::type_map_group::TypeMapGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeMapField {
    Column(String),
    Group(TypeMapGroup),
}

impl TypeMapField {
    pub fn column(source: impl Into<String>) -> Self {
        Self::Column(source.into())
    }
}
