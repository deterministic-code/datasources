use super::routine_parameter::RoutineParameter;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct CreateBulkCreateStoredProcedureStatement {
    name: String,
    table: String,
    columns: Vec<RoutineParameter>,
    payload: RoutineParameter,
    key: String,
    key_type: String,
}

impl CreateBulkCreateStoredProcedureStatement {
    pub fn new(
        name: impl AsRef<str>,
        table: impl AsRef<str>,
        columns: Vec<RoutineParameter>,
        payload: RoutineParameter,
        key: impl AsRef<str>,
        key_type: impl AsRef<str>,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            table: table.as_ref().to_string(),
            columns,
            payload,
            key: key.as_ref().to_string(),
            key_type: key_type.as_ref().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn columns(&self) -> &[RoutineParameter] {
        &self.columns
    }

    pub fn payload(&self) -> &RoutineParameter {
        &self.payload
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn key_type(&self) -> &str {
        &self.key_type
    }
}

impl Statement for CreateBulkCreateStoredProcedureStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_create_bulk_create_stored_procedure(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateBulkCreateStoredProcedureStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
