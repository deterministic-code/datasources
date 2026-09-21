#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotNull {
    pub name: Option<String>,
}

impl NotNull {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
        }
    }
}

impl Default for NotNull {
    fn default() -> Self {
        Self::new()
    }
}
