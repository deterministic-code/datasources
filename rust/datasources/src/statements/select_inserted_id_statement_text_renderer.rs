use super::select_inserted_id_statement::SelectInsertedIdStatement;

pub struct SelectInsertedIdStatementTextRenderer;

impl SelectInsertedIdStatementTextRenderer {
    pub fn render(&self, _statement: &SelectInsertedIdStatement) -> String {
        panic!("provider must implement SelectInsertIdStatement")
    }
}

impl Default for SelectInsertedIdStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
