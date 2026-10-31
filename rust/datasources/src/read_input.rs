use crate::IWhere;
use crate::statement::Statement;

impl ReadInput for () {}

pub trait ReadInput: Send + Sync {
    fn as_statement(&self) -> Option<&dyn Statement> {
        None
    }

    fn as_where(&self) -> Option<&dyn IWhere> {
        None
    }
}

pub fn where_of<'a>(query: Option<&'a dyn ReadInput>) -> Option<&'a dyn IWhere> {
    let input = query?;
    input
        .as_where()
        .or_else(|| input.as_statement().and_then(Statement::predicate))
}
