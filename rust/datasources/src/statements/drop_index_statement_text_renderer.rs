use super::drop_index_statement::DropIndexStatement;

pub struct DropIndexStatementTextRenderer;

impl DropIndexStatementTextRenderer {
    pub fn render(&self, statement: &DropIndexStatement) -> String {
        format!("DROP INDEX IF EXISTS {}", self.quote(statement.name()))
    }

    fn quote(&self, name: &str) -> String {
        name.to_string()
    }
}
