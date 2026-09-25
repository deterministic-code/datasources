use super::text::Text;
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

pub trait Statement: Text {
    fn as_any(&self) -> &dyn Any;
    fn kind(&self) -> StatementKind {
        StatementKind::Other
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        None
    }
}
