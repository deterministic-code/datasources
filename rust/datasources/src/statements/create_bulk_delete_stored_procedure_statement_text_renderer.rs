use super::create_bulk_delete_stored_procedure_statement::CreateBulkDeleteStoredProcedureStatement;

pub struct CreateBulkDeleteStoredProcedureStatementTextRenderer;

impl CreateBulkDeleteStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateBulkDeleteStoredProcedureStatement) -> String {
        String::new()
    }
}
