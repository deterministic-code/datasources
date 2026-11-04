use super::bulk_insert_statement::BulkInsertStatement;

pub struct BulkInsertStatementTextRenderer;

impl BulkInsertStatementTextRenderer {
    pub fn quote(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn parameter(&self, name: &str) -> String {
        format!("${name}")
    }

    pub fn render(&self, statement: &BulkInsertStatement) -> String {
        Self::render_with(statement, |name| self.quote(name), |name| self.parameter(name))
    }

    pub fn render_with(
        statement: &BulkInsertStatement,
        quote: impl Fn(&str) -> String,
        parameter: impl Fn(&str) -> String,
    ) -> String {
        let values = (0..statement.rows().len())
            .map(|index| {
                format!(
                    "({})",
                    statement
                        .columns()
                        .iter()
                        .map(|column| parameter(&format!("{column}__{index}")))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES {values}",
            quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for BulkInsertStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
