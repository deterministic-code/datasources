use datasources::CreateTriggerStatement;

pub struct MysqlCreateTriggerStatementRenderer;

impl MysqlCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "CREATE TRIGGER {} BEFORE UPDATE ON {}\nFOR EACH ROW SET NEW.{} = {}",
            statement.name(),
            statement.table(),
            statement.updated_column(),
            statement.utc_now(),
        )
    }
}
