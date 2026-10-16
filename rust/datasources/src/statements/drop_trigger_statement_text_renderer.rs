use super::drop_trigger_statement::DropTriggerStatement;

pub struct DropTriggerStatementTextRenderer;

impl DropTriggerStatementTextRenderer {
    pub fn render(&self, statement: &DropTriggerStatement) -> String {
        format!("DROP TRIGGER IF EXISTS {}", self.quote(statement.name()))
    }

    fn quote(&self, name: &str) -> String {
        name.to_string()
    }
}
