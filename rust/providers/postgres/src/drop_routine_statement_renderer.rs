use datasources::DropRoutineStatement;

pub struct PostgresDropRoutineStatementRenderer;

impl PostgresDropRoutineStatementRenderer {
    pub fn render(&self, statement: &DropRoutineStatement) -> String {
        format!("DROP FUNCTION IF EXISTS {}", statement.name())
    }
}
