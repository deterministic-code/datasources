use super::bulk_delete_statement::BulkDeleteStatement;

pub struct BulkDeleteStatementTextRenderer;

impl BulkDeleteStatementTextRenderer {
    pub fn quote(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn parameter(&self, name: &str) -> String {
        format!("${name}")
    }

    pub fn render(&self, statement: &BulkDeleteStatement) -> String {
        Self::render_with(statement, |name| self.quote(name), |name| self.parameter(name))
    }

    pub fn render_with(
        statement: &BulkDeleteStatement,
        quote: impl Fn(&str) -> String,
        parameter: impl Fn(&str) -> String,
    ) -> String {
        if statement.ids().is_empty() {
            return format!("DELETE FROM {} WHERE 1 = 1", quote(statement.table()));
        }
        if statement.ids().len() == 1 {
            let column = &statement.ids()[0];
            let names = (0..statement.rows().len())
                .map(|index| parameter(&format!("{column}__{index}")))
                .collect::<Vec<_>>()
                .join(", ");
            return format!(
                "DELETE FROM {} WHERE {} IN ({names})",
                quote(statement.table()),
                quote(column)
            );
        }
        let tuples = (0..statement.rows().len())
            .map(|index| {
                format!(
                    "({})",
                    statement
                        .ids()
                        .iter()
                        .map(|column| parameter(&format!("{column}__{index}")))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "DELETE FROM {} WHERE ({}) IN ({tuples})",
            quote(statement.table()),
            statement
                .ids()
                .iter()
                .map(|column| quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for BulkDeleteStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
