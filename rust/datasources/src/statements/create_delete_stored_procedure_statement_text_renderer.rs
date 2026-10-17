use super::create_delete_stored_procedure_statement::CreateDeleteStoredProcedureStatement;

pub struct CreateDeleteStoredProcedureStatementTextRenderer;

impl CreateDeleteStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateDeleteStoredProcedureStatement) -> String {
        String::new()
    }
}
