use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;

pub struct StoredProcedureStatement {
    text: String,
}

impl StoredProcedureStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(name: impl AsRef<str>, parameters: &[Parameter]) -> Self {
        Self {
            text: format!(
                "CALL {}({})",
                name.as_ref(),
                parameters
                    .iter()
                    .map(|field| format!("${}", field.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Text for StoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for StoredProcedureStatement {}
