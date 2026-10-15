use super::drop_table_statement::DropTableStatement;

pub struct DropTableStatementTextRenderer;

impl DropTableStatementTextRenderer {
    pub fn render(&self, statement: &DropTableStatement) -> String {
        format!("DROP TABLE IF EXISTS {}", statement.table())
    }
}
