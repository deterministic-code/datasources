use super::drop_routine_statement::DropRoutineStatement;

pub struct DropRoutineStatementTextRenderer;

impl DropRoutineStatementTextRenderer {
    pub fn render(&self, _statement: &DropRoutineStatement) -> String {
        String::new()
    }
}
