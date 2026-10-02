use datasources::SelectInsertedIdStatement;

pub struct PostgresSelectInsertedIdStatementTextRenderer;

impl PostgresSelectInsertedIdStatementTextRenderer {
    pub fn render(&self, statement: &SelectInsertedIdStatement) -> String {
        format!("SELECT lastval() AS {}", statement.column)
    }
}

impl Default for PostgresSelectInsertedIdStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
