use crate::query::Query;
use crate::statement::Statement;
use crate::where_trait::Where;

pub struct TextQuery {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl TextQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self {
        Self {
            text: text.into(),
            where_clause: Some(Box::new(where_clause)),
        }
    }
}

impl Statement for TextQuery {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Query for TextQuery {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
