use super::text::Text;
use crate::i_where::IWhere;
use crate::parameter::Parameter;
use std::any::Any;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementKind {
    Query,
    Procedure,
    Insert,
    Update,
    Delete,
    Other,
}

pub trait Statement: Text {
    fn as_any(&self) -> &dyn Any;
    fn parameters(&self) -> Vec<Parameter> {
        Vec::new()
    }
    fn kind(&self) -> StatementKind {
        StatementKind::Other
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        None
    }
}
