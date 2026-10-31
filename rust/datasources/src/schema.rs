use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaColumn {
    pub type_name: String,
    pub nullable: bool,
    pub has_default: bool,
}

impl SchemaColumn {
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            nullable: false,
            has_default: false,
        }
    }
}

impl From<&str> for SchemaColumn {
    fn from(type_name: &str) -> Self {
        Self::new(type_name)
    }
}

impl From<String> for SchemaColumn {
    fn from(type_name: String) -> Self {
        Self::new(type_name)
    }
}

#[derive(Clone, Default)]
pub struct Schema {
    pub columns: HashMap<String, SchemaColumn>,
}

impl Schema {
    pub fn new(columns: HashMap<String, SchemaColumn>) -> Self {
        Self { columns }
    }

    pub fn types(&self) -> HashMap<String, String> {
        self.columns
            .iter()
            .map(|(name, column)| (name.clone(), column.type_name.clone()))
            .collect()
    }
}
