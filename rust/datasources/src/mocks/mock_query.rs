use crate::query::Query;
use crate::statement::Statement;
use crate::statements::text::Text;
use crate::where_trait::Where;
use std::sync::Mutex;

pub struct MockQuery {
    text: String,
    where_clause: Option<Box<dyn Where>>,
    calls: Mutex<usize>,
}

impl MockQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
            calls: Mutex::new(0),
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self {
        Self {
            text: text.into(),
            where_clause: Some(Box::new(where_clause)),
            calls: Mutex::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        *self.calls.lock().expect("mock")
    }
}

impl Text for MockQuery {
    fn get_text(&self) -> String {
        *self.calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}

impl crate::ReadInput for MockQuery {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for MockQuery {
    fn kind(&self) -> crate::statements::StatementKind {
        crate::statements::StatementKind::Query
    }
    fn predicate(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

impl Query for MockQuery {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
