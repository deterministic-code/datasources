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

impl ForeignKey {
    pub fn new(
        name: impl Into<String>,
        columns: &[&str],
        references_table: impl Into<String>,
        references_columns: &[&str],
    ) -> Self {
        Self {
            name: name.into(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
            references_table: references_table.into(),
            references_columns: references_columns
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            on_delete: None,
            on_update: None,
        }
    }

    pub fn with_on_delete(self, action: ReferentialAction) -> Self {
        Self {
            on_delete: Some(action),
            ..self
        }
    }

    pub fn with_on_update(self, action: ReferentialAction) -> Self {
        Self {
            on_update: Some(action),
            ..self
        }
    }
}
