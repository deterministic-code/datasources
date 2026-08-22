use crate::query::Query;

pub struct TextQuery {
    text: String,
}

impl TextQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Query for TextQuery {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}
