use super::create_read_stored_procedure_statement::CreateReadStoredProcedureStatement;

pub struct CreateReadStoredProcedureStatementTextRenderer;

impl CreateReadStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateReadStoredProcedureStatement) -> String {
        String::new()
    }
}
