#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueConstraint {
    pub name: String,
    pub columns: Vec<String>,
}

impl UniqueConstraint {
    pub fn new(name: impl Into<String>, columns: &[&str]) -> Self {
        Self {
            name: name.into(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
        }
    }
}
