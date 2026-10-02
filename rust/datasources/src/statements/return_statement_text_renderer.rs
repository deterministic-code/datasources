use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;
use super::return_statement::ReturnStatement;

pub struct ReturnStatementTextRenderer;

impl ReturnStatementTextRenderer {
    pub fn render(&self, statement: &ReturnStatement) -> String {
        format!("SELECT {}({})", statement.name(), args(statement.parameters()))
    }
}

impl Default for ReturnStatementTextRenderer {
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
