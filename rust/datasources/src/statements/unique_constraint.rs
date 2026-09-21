#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueConstraint {
    pub name: String,
    pub columns: Vec<String>,
}
