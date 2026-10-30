use crate::SqliteDriver;
use datasources::CreateTriggerStatement;

pub struct SqliteCreateTriggerStatementRenderer;

impl SqliteCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        let table = SqliteDriver::quote(statement.table());
        let name = SqliteDriver::quote(statement.name());
        let updated = SqliteDriver::quote(statement.updated_column());
        let key = SqliteDriver::quote(statement.key_column());
        format!(
            "CREATE TRIGGER {name}\nAFTER UPDATE ON {table}\nFOR EACH ROW\nWHEN NEW.{updated} IS OLD.{updated}\nBEGIN\n  UPDATE {table} SET {updated} = {} WHERE {key} = OLD.{key};\nEND",
            SqliteDriver::utc_now(),
        )
    }
}
