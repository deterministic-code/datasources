use crate::MysqlDriver;
use datasources::CreateTriggerStatement;

pub struct MysqlCreateTriggerStatementRenderer;

impl MysqlCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE TRIGGER {} BEFORE UPDATE ON {}\nFOR EACH ROW SET NEW.{} = {}",
            MysqlDriver::quote(statement.name()),
            MysqlDriver::quote(statement.table()),
            MysqlDriver::quote(statement.updated_column()),
            MysqlDriver::utc_now(),
        )
    }
}
