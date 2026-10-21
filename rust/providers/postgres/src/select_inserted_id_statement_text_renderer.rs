use datasources::SelectInsertedIdStatement;

pub struct PostgresSelectInsertedIdStatementTextRenderer;

impl PostgresSelectInsertedIdStatementTextRenderer {
    pub fn render(&self, statement: &SelectInsertedIdStatement) -> String {
        let table = statement.table.as_str();
        let column = statement.column.as_str();
        format!(
            "SELECT setval(pg_get_serial_sequence('{table}', '{column}'), COALESCE((SELECT MAX(\"{column}\") FROM {table}), 1)) AS {column}"
        )
    }
}

impl Default for PostgresSelectInsertedIdStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
