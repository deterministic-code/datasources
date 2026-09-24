use crate::statement::Statement;
use crate::where_trait::Where;

pub trait ReadInput: Send + Sync {
    fn as_statement(&self) -> Option<&dyn Statement> {
        None
    }

    fn as_where(&self) -> Option<&dyn Where> {
        None
    }
}

pub(crate) struct StatementRead<'a>(pub &'a dyn Statement);

impl ReadInput for StatementRead<'_> {
    fn as_statement(&self) -> Option<&dyn Statement> {
        Some(self.0)
    }
}

#[cfg(test)]
pub fn where_of<'a>(query: Option<&'a dyn ReadInput>) -> Option<&'a dyn Where> {
    let input = query?;
    input
        .as_where()
        .or_else(|| input.as_statement().and_then(Statement::predicate))
}
