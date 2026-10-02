use super::statement_text_renderer::StatementTextRenderer;
use crate::i_where::IWhere;
use std::any::Any;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Select,
    Procedure,
    Insert,
    Update,
    Delete,
    Other,
}

pub trait Statement: Send + Sync {
    fn accept(&self, text: &StatementTextRenderer) -> String;
    fn as_any(&self) -> &dyn Any;
    fn kind(&self) -> StatementKind {
        StatementKind::Other
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        None
    }
}
