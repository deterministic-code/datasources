use super::statement::Statement;

pub trait CreateTableStatement: Statement {}

pub struct TextCreateTableStatement {
    text: String,
}

impl TextCreateTableStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextCreateTableStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl CreateTableStatement for TextCreateTableStatement {}
