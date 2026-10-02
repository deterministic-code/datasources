use super::statement::{Statement, StatementKind};
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;

pub struct ExecuteStoredProcedureStatement {
    name: String,
    parameters: Vec<Parameter>,
}

impl ExecuteStoredProcedureStatement {
    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        Self {
            name: name.as_ref().to_string(),
            parameters: parameters.to_vec(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }
}

impl crate::ReadInput for ExecuteStoredProcedureStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for ExecuteStoredProcedureStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_execute_stored_procedure(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> StatementKind {
        StatementKind::Procedure
    }
}
