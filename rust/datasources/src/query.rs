use crate::i_where::IWhere;
use crate::statement::Statement;

pub trait Query: Statement {
    fn where_clause(&self) -> Option<&dyn IWhere>;
}
