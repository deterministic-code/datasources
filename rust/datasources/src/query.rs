use crate::statement::Statement;
use crate::where_trait::Where;

pub trait Query: Statement {
    fn where_clause(&self) -> Option<&dyn Where>;
}
