use datasources::CreateTriggerStatement;

pub struct OracleCreateTriggerStatementRenderer;

impl OracleCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE OR REPLACE TRIGGER {}\nBEFORE UPDATE ON {}\nFOR EACH ROW\nBEGIN\n  :NEW.{} := {};\nEND;\n/",
            statement.name(),
            statement.table(),
            statement.updated_column(),
            statement.utc_now(),
        )
    }
}
