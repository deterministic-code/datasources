use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DropRoutineStatement {
    name: String,
}

impl DropRoutineStatement {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Statement for DropRoutineStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_drop_routine(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for DropRoutineStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
