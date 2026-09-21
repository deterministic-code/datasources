#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDefault {
    pub value: String,
    pub name: Option<String>,
}

impl ColumnDefault {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            name: None,
        }
    }

    pub fn named(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            name: Some(name.into()),
        }
    }
}
