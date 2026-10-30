use crate::converters::SpecField;
use crate::statements::{ColumnDefault, NotNull};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub type_name: String,
    pub default: Option<ColumnDefault>,
    pub not_null: Option<NotNull>,
    pub identity: bool,
    pub field: Option<SpecField>,
}

impl Column {
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            default: None,
            not_null: None,
            identity: false,
            field: None,
        }
    }

    pub fn field(self, field: SpecField) -> Self {
        Self {
            field: Some(field),
            ..self
        }
    }

    pub fn identity(self) -> Self {
        Self {
            identity: true,
            ..self
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
