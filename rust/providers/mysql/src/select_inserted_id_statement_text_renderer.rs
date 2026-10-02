use datasources::SelectInsertedIdStatement;

pub struct MysqlSelectInsertedIdStatementTextRenderer;

impl MysqlSelectInsertedIdStatementTextRenderer {
    pub fn render(&self, statement: &SelectInsertedIdStatement) -> String {
        format!("SELECT LAST_INSERT_ID() AS {}", statement.column)
    }
}

impl Default for MysqlSelectInsertedIdStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
