use super::create_nested_read_stored_procedure_statement::CreateNestedReadStoredProcedureStatement;

pub struct CreateNestedReadStoredProcedureStatementTextRenderer;

impl CreateNestedReadStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateNestedReadStoredProcedureStatement) -> String {
        String::new()
    }
}
