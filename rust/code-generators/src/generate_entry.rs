#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerateEntry {
    pub filename: String,
    pub content: String,
}

impl GenerateEntry {
    pub fn new(filename: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            content: content.into(),
        }
    }
}
