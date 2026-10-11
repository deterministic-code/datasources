use super::bulk_insert_statement::BulkInsertStatement;

pub struct BulkInsertStatementTextRenderer;

impl BulkInsertStatementTextRenderer {
    pub fn render(&self, statement: &BulkInsertStatement) -> String {
        let values = (0..statement.rows().len())
            .map(|index| {
                format!(
                    "({})",
                    statement
                        .columns()
                        .iter()
                        .map(|column| format!("${column}__{index}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES {values}",
            statement.table(),
            statement.columns().join(", ")
        )
    }
}

impl Default for BulkInsertStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
