use datasources::DropRoutineStatement;

pub struct SqlServerDropRoutineStatementRenderer;

impl SqlServerDropRoutineStatementRenderer {
    pub fn render(&self, statement: &DropRoutineStatement) -> String {
        format!("DROP PROCEDURE IF EXISTS {}", statement.name())
    }
}
