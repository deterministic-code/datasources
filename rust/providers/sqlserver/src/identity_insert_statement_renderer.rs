use datasources::{IdentityInsertStatement, IdentityInsertStatementTextRenderer};

pub struct SqlServerIdentityInsertStatementRenderer;

impl SqlServerIdentityInsertStatementRenderer {
    pub fn render(&self, statement: &IdentityInsertStatement) -> String {
        let inserts = IdentityInsertStatementTextRenderer.render(statement);
        if !statement.identity() || statement.inserts().is_empty() {
            return inserts;
        }
        let table = statement.inserts()[0].table();
        format!("SET IDENTITY_INSERT {table} ON;\n{inserts}\nSET IDENTITY_INSERT {table} OFF")
    }
}
