use crate::i_select_statement::ISelectStatement;
use crate::i_where::IWhere;
use crate::statement::Statement;
use crate::statements::text::Text;
use std::sync::Mutex;

pub struct MockSelectStatement {
    text: String,
    where_clause: Option<Box<dyn IWhere>>,
    calls: Mutex<usize>,
}

impl MockSelectStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
            calls: Mutex::new(0),
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl IWhere + 'static) -> Self {
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

impl Text for MockSelectStatement {
    fn get_text(&self) -> String {
        *self.calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}

impl crate::ReadInput for MockSelectStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for MockSelectStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> crate::statements::StatementKind {
        crate::statements::StatementKind::Select
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

impl ISelectStatement for MockSelectStatement {
    fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}
