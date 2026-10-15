use datasources::{IdentityInsertStatement, IdentityInsertStatementTextRenderer};

pub struct PostgresIdentityInsertStatementRenderer;

impl PostgresIdentityInsertStatementRenderer {
    pub fn render(&self, statement: &IdentityInsertStatement) -> String {
        let inserts = IdentityInsertStatementTextRenderer.render(statement);
        if !statement.identity() || statement.inserts().is_empty() {
            return inserts;
        }
        format!(
            "{inserts}\nSELECT setval(pg_get_serial_sequence('{}', '{}'), (SELECT MAX(\"{}\") FROM {}))",
            statement.table(),
            statement.id_column(),
            statement.id_column(),
            statement.inserts()[0].table(),
        )
    }
}
