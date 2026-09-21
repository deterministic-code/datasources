use super::statement::Statement;
use crate::where_trait::Where;

pub trait DeleteStatement: Statement {
    fn where_clause(&self) -> Option<&dyn Where>;
}

pub struct TextDeleteStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl TextDeleteStatement {
    pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }
}

impl Statement for TextDeleteStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl DeleteStatement for TextDeleteStatement {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
