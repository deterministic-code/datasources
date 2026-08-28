use crate::statement::Statement;
use crate::where_trait::Where;

pub struct TextWhere {
    text: String,
}

impl TextWhere {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextWhere {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Where for TextWhere {}
