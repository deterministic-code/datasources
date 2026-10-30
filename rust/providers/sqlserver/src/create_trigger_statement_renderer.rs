use crate::SqlServerDriver;
use datasources::CreateTriggerStatement;

pub struct SqlServerCreateTriggerStatementRenderer;

impl SqlServerCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        let table = SqlServerDriver::quote(statement.table());
        let updated = SqlServerDriver::quote(statement.updated_column());
        let key = SqlServerDriver::quote(statement.key_column());
        format!(
            "CREATE TRIGGER {} ON {table}\nAFTER UPDATE AS\nBEGIN\n  SET NOCOUNT ON;\n  UPDATE {table} SET {updated} = {}\n  FROM {table} t INNER JOIN inserted i ON t.{key} = i.{key};\nEND",
            SqlServerDriver::quote(statement.name()),
            SqlServerDriver::utc_now(),
        )
    }
}
