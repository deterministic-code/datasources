use super::create_update_stored_procedure_statement::CreateUpdateStoredProcedureStatement;

pub struct CreateUpdateStoredProcedureStatementTextRenderer;

impl CreateUpdateStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateUpdateStoredProcedureStatement) -> String {
        String::new()
    }
}
