use crate::statements::{ColumnDefault, NotNull};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub type_name: String,
    pub default: Option<ColumnDefault>,
    pub not_null: Option<NotNull>,
}

impl Column {
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            default: None,
            not_null: None,
        }
    }

    pub fn default(self, default: ColumnDefault) -> Self {
        Self {
            default: Some(default),
            ..self
        }
    }

    pub fn not_null(self, not_null: NotNull) -> Self {
        Self {
            not_null: Some(not_null),
            ..self
        }
    }
}
