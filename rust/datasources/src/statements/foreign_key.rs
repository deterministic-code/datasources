#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferentialAction {
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub references_table: String,
    pub references_columns: Vec<String>,
    pub on_delete: Option<ReferentialAction>,
    pub on_update: Option<ReferentialAction>,
}
