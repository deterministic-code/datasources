use datasources::SelectInsertedIdStatement;

pub struct SqliteSelectInsertedIdStatementTextRenderer;

impl SqliteSelectInsertedIdStatementTextRenderer {
    pub fn render(&self, statement: &SelectInsertedIdStatement) -> String {
        format!("SELECT last_insert_rowid() AS {}", statement.column)
    }
}

impl Default for SqliteSelectInsertedIdStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
