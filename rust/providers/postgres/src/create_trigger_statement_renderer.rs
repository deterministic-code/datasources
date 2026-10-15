use datasources::CreateTriggerStatement;

pub struct PostgresCreateTriggerStatementRenderer;

impl PostgresCreateTriggerStatementRenderer {
    pub fn render(&self, statement: &CreateTriggerStatement) -> String {
        format!(
            "-- Shared updated_at function\nCREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$\nBEGIN\n  NEW.{} = NOW() AT TIME ZONE 'UTC';\n  RETURN NEW;\nEND;\n$$ LANGUAGE plpgsql;\nCREATE TRIGGER {} BEFORE UPDATE ON {} FOR EACH ROW EXECUTE FUNCTION set_updated_at()",
            statement.updated_column(),
            statement.name(),
            statement.table(),
        )
    }
}
