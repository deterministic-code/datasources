use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaColumn {
    pub type_name: String,
    pub nullable: bool,
    pub has_default: bool,
    pub is_id: bool,
}

impl SchemaColumn {
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            nullable: false,
            has_default: false,
            is_id: false,
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

    pub fn empty() -> &'static Schema {
        static EMPTY: OnceLock<Schema> = OnceLock::new();
        EMPTY.get_or_init(Schema::default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_uses_is_id() {
        let schema = Schema::new(HashMap::from([
            (
                "Key".into(),
                SchemaColumn {
                    type_name: "string".into(),
                    is_id: true,
                    ..SchemaColumn::new("string")
                },
            ),
            ("Name".into(), SchemaColumn::new("string")),
        ]));
        assert_eq!(crate::identity::ids_from_schema(&schema), vec!["Key".to_string()]);
        assert_eq!(schema.types().get("Name"), Some(&"string".to_string()));
    }
}
