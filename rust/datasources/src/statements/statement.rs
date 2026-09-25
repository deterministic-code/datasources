use super::text::Text;
use crate::parameter::Parameter;
use crate::where_trait::Where;
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
    fn predicate(&self) -> Option<&dyn Where> {
        None
    }
}
