use crate::ISelectStatement;
use crate::IWhere;
use crate::statement::Statement;
use crate::StatementTextRenderer;
use std::sync::Mutex;

pub struct MockSelectStatement {
    text: String,
    where_clause: Option<Box<dyn IWhere>>,
    accept_calls: Mutex<usize>,
}

impl MockSelectStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
            accept_calls: Mutex::new(0),
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl IWhere + 'static) -> Self {
        Self {
            text: text.into(),
            where_clause: Some(Box::new(where_clause)),
            accept_calls: Mutex::new(0),
        }
    }

    pub fn accept_calls(&self) -> usize {
        *self.accept_calls.lock().expect("mock")
    }
}

impl crate::ReadInput for MockSelectStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for MockSelectStatement {
    fn accept(&self, _text: &StatementTextRenderer) -> String {
        *self.accept_calls.lock().expect("mock") += 1;
        self.text.clone()
    }

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
