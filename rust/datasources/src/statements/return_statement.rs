use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;

pub struct ReturnStatement {
    text: String,
}

impl ReturnStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        Self {
            text: format!("SELECT {}({})", name.as_ref(), args(parameters)),
        }
    }
}

impl Text for ReturnStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for ReturnStatement {}

fn args(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .filter(|parameter| parameter.direction != ParameterDirection::ReturnValue)
        .map(|parameter| format!("${}", parameter.name))
        .collect::<Vec<_>>()
        .join(", ")
}
