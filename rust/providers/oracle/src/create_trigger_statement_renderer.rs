use crate::OracleDriver;
use datasources::CreateTriggerStatement;

pub struct OracleCreateTriggerStatementRenderer;

impl OracleCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE OR REPLACE TRIGGER {}\nBEFORE UPDATE ON {}\nFOR EACH ROW\nBEGIN\n  :NEW.{} := {};\nEND;\n/",
            OracleDriver::quote(statement.name()),
            OracleDriver::quote(statement.table()),
            OracleDriver::quote(statement.updated_column()),
            OracleDriver::utc_now(),
        )
    }
}
