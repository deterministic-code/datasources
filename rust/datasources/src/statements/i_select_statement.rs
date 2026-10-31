use super::i_where::IWhere;
use crate::statement::Statement;

pub trait ISelectStatement: Statement {
    fn where_clause(&self) -> Option<&dyn IWhere>;
}
