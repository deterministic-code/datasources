use super::create_routine_statement::CreateRoutineStatement;

pub struct CreateRoutineStatementTextRenderer;

impl CreateRoutineStatementTextRenderer {
    pub fn render(&self, _statement: &CreateRoutineStatement) -> String {
        String::new()
    }
}
