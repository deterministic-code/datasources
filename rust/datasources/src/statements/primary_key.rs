#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryKey {
    pub name: String,
    pub columns: Vec<String>,
}
