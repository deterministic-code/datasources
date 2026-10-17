use super::create_bulk_create_stored_procedure_statement::CreateBulkCreateStoredProcedureStatement;

pub struct CreateBulkCreateStoredProcedureStatementTextRenderer;

impl CreateBulkCreateStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateBulkCreateStoredProcedureStatement) -> String {
        String::new()
    }
}
