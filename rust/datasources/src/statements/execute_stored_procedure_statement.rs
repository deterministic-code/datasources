use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;

pub struct ExecuteStoredProcedureStatement {
    text: String,
}

impl ExecuteStoredProcedureStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        Self {
            text: format!("CALL {}({})", name.as_ref(), args(parameters)),
        }
    }
}

impl Text for ExecuteStoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for ExecuteStoredProcedureStatement {}

fn args(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .filter(|parameter| parameter.direction != ParameterDirection::ReturnValue)
        .map(|parameter| format!("${}", parameter.name))
        .collect::<Vec<_>>()
        .join(", ")
}
