use super::insert_statement::InsertStatement;

pub struct InsertStatementTextRenderer;

impl InsertStatementTextRenderer {
    pub fn quote(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn parameter(&self, name: &str) -> String {
        format!("${name}")
    }

    pub fn render(&self, statement: &InsertStatement) -> String {
        Self::render_with(statement, |name| self.quote(name), |name| self.parameter(name))
    }

    pub fn render_with(
        statement: &InsertStatement,
        quote: impl Fn(&str) -> String,
        parameter: impl Fn(&str) -> String,
    ) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            quote(statement.table()),
            statement
                .fields()
                .iter()
                .map(|field| quote(field.name.as_str()))
                .collect::<Vec<_>>()
                .join(", "),
            statement
                .fields()
                .iter()
                .map(|field| parameter(&field.name))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for InsertStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
