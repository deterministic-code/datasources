use super::bulk_delete_statement::BulkDeleteStatement;

pub struct BulkDeleteStatementTextRenderer;

impl BulkDeleteStatementTextRenderer {
    pub fn render(&self, statement: &BulkDeleteStatement) -> String {
        if statement.ids().is_empty() {
            return format!("DELETE FROM {} WHERE 1 = 1", statement.table());
        }
        if statement.ids().len() == 1 {
            let column = &statement.ids()[0];
            let names = (0..statement.rows().len())
                .map(|index| format!("${column}__{index}"))
                .collect::<Vec<_>>()
                .join(", ");
            return format!(
                "DELETE FROM {} WHERE {column} IN ({names})",
                statement.table()
            );
        }
        let tuples = (0..statement.rows().len())
            .map(|index| {
                format!(
                    "({})",
                    statement
                        .ids()
                        .iter()
                        .map(|column| format!("${column}__{index}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "DELETE FROM {} WHERE ({}) IN ({tuples})",
            statement.table(),
            statement.ids().join(", ")
        )
    }
}

impl Default for BulkDeleteStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
