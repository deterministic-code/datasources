use super::create_trigger_statement::CreateTriggerStatement;

pub struct CreateTriggerStatementTextRenderer;

impl CreateTriggerStatementTextRenderer {
    pub fn render(&self, _statement: &CreateTriggerStatement) -> String {
        String::new()
    }
}
