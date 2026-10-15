use datasources::CreateTriggerStatement;

pub struct SqlServerCreateTriggerStatementRenderer;

impl SqlServerCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE TRIGGER {} ON {}\nAFTER UPDATE AS\nBEGIN\n  SET NOCOUNT ON;\n  UPDATE {} SET {} = {}\n  FROM {} t INNER JOIN inserted i ON t.{} = i.{};\nEND",
            statement.name(),
            statement.table(),
            statement.table(),
            statement.updated_column(),
            statement.utc_now(),
            statement.table(),
            statement.key_column(),
            statement.key_column(),
        )
    }
}
