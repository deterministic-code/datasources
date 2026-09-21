use super::statement::Statement;

pub trait InsertStatement: Statement {}

pub struct TextInsertStatement {
    text: String,
}

impl TextInsertStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextInsertStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl InsertStatement for TextInsertStatement {}
