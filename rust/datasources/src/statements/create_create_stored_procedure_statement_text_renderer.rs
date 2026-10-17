use super::create_create_stored_procedure_statement::CreateCreateStoredProcedureStatement;

pub struct CreateCreateStoredProcedureStatementTextRenderer;

impl CreateCreateStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateCreateStoredProcedureStatement) -> String {
        String::new()
    }
}
