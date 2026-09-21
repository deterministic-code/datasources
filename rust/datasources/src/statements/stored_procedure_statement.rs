use super::statement::Statement;

pub trait StoredProcedureStatement: Statement {}

pub struct TextStoredProcedureStatement {
    text: String,
}

impl TextStoredProcedureStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextStoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl StoredProcedureStatement for TextStoredProcedureStatement {}
