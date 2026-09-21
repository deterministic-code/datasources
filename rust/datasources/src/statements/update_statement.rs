use super::statement::Statement;
use crate::where_trait::Where;

pub trait UpdateStatement: Statement {
    fn where_clause(&self) -> Option<&dyn Where>;
}

pub struct TextUpdateStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl TextUpdateStatement {
    pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }
}

impl Statement for TextUpdateStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl UpdateStatement for TextUpdateStatement {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
