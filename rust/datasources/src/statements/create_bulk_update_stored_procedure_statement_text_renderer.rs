use super::create_bulk_update_stored_procedure_statement::CreateBulkUpdateStoredProcedureStatement;

pub struct CreateBulkUpdateStoredProcedureStatementTextRenderer;

impl CreateBulkUpdateStoredProcedureStatementTextRenderer {
    pub fn render(&self, _statement: &CreateBulkUpdateStoredProcedureStatement) -> String {
        String::new()
    }
}
