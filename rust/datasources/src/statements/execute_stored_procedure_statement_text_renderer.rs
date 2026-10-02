use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;
use super::execute_stored_procedure_statement::ExecuteStoredProcedureStatement;

pub struct ExecuteStoredProcedureStatementTextRenderer;

impl ExecuteStoredProcedureStatementTextRenderer {
    pub fn render(&self, statement: &ExecuteStoredProcedureStatement) -> String {
        format!("CALL {}({})", statement.name(), args(statement.parameters()))
    }
}

impl Default for ExecuteStoredProcedureStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}

fn args(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .filter(|parameter| parameter.direction != ParameterDirection::ReturnValue)
        .map(|parameter| format!("${}", parameter.name))
        .collect::<Vec<_>>()
        .join(", ")
}
