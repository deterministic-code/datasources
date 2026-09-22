use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GroupView {
    pub id: i64,
    pub name: String,
    pub is_enabled: bool,
}
