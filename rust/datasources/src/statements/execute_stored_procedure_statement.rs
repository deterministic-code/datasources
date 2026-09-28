use super::statement::{Statement, StatementKind};
use super::text::Text;
use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;

pub struct ExecuteStoredProcedureStatement {
    text: String,
    pub name: Option<String>,
}

impl ExecuteStoredProcedureStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            name: None,
        }
    }

    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        let name = name.as_ref().to_string();
        Self {
            text: format!("CALL {name}({})", args(parameters)),
            name: Some(name),
        }
    }
}

impl Text for ExecuteStoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl crate::ReadInput for ExecuteStoredProcedureStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for ExecuteStoredProcedureStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> StatementKind {
        StatementKind::Procedure
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
