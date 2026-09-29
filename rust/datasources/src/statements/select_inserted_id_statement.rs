use super::statement::Statement;
use super::text::Text;

pub struct SelectInsertedIdStatement {
    pub table: String,
    pub column: String,
}

impl SelectInsertedIdStatement {
    pub fn new(table: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            column: column.into(),
        }
    }
}

impl Text for SelectInsertedIdStatement {
    fn get_text(&self) -> String {
        panic!("provider must implement SelectInsertIdStatement");
    }
}

impl Statement for SelectInsertedIdStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
