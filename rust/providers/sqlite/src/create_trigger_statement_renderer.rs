use datasources::CreateTriggerStatement;

pub struct SqliteCreateTriggerStatementRenderer;

impl SqliteCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE TRIGGER {}\nAFTER UPDATE ON {}\nFOR EACH ROW\nWHEN NEW.{} IS OLD.{}\nBEGIN\n  UPDATE {} SET {} = {} WHERE {} = OLD.{};\nEND",
            statement.name(),
            statement.table(),
            statement.updated_column(),
            statement.updated_column(),
            statement.table(),
            statement.updated_column(),
            statement.utc_now(),
            statement.key_column(),
            statement.key_column(),
        )
    }
}
