use crate::PostgresDriver;
use datasources::CreateTriggerStatement;

pub struct PostgresCreateTriggerStatementRenderer;

impl PostgresCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        let updated = PostgresDriver::quote(statement.updated_column());
        format!(
            "-- Shared updated_at function\nCREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$\nBEGIN\n  NEW.{updated} = {};\n  RETURN NEW;\nEND;\n$$ LANGUAGE plpgsql;\nCREATE TRIGGER {} BEFORE UPDATE ON {} FOR EACH ROW EXECUTE FUNCTION set_updated_at()",
            PostgresDriver::utc_now(),
            PostgresDriver::quote(statement.name()),
            PostgresDriver::quote(statement.table()),
        )
    }
}
