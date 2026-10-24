use crate::nested_select_relation::NestedSelectRelation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedSelectType {
    pub name: String,
    pub key: String,
    pub properties: Vec<String>,
    pub relations: Vec<NestedSelectRelation>,
}

impl NestedSelectType {
    pub fn new(
        name: impl Into<String>,
        key: impl Into<String>,
        properties: Vec<String>,
        relations: Vec<NestedSelectRelation>,
    ) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
            properties,
            relations,
        }
    }
}
