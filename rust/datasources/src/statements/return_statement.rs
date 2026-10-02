use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;

pub struct ReturnStatement {
    name: String,
    parameters: Vec<Parameter>,
}

impl ReturnStatement {
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

impl Statement for ReturnStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_return(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for ReturnStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
