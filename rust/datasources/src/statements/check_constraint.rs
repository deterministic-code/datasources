#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckConstraint {
    pub name: String,
    pub expression: String,
}
