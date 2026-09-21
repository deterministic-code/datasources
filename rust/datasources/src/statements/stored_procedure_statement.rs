use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;

pub struct StoredProcedureStatement {
    text: String,
}

impl StoredProcedureStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        Self {
            text: format!("CALL {}({})", name.as_ref(), args(parameters)),
        }
    }
}

impl Text for StoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for StoredProcedureStatement {}

fn args(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .filter(|parameter| parameter.direction != ParameterDirection::ReturnValue)
        .map(|parameter| format!("${}", parameter.name))
        .collect::<Vec<_>>()
        .join(", ")
}
