use datasources::DropRoutineStatement;

pub struct MysqlDropRoutineStatementRenderer;

impl MysqlDropRoutineStatementRenderer {
    pub fn render(&self, statement: &DropRoutineStatement) -> String {
        format!("DROP PROCEDURE IF EXISTS {}", statement.name())
    }
}
