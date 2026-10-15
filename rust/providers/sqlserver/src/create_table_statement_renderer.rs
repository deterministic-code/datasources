use datasources::{Column, CreateTableStatementTextRenderer};

pub struct SqlServerCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqlServerCreateTableStatementRenderer {
    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " IDENTITY(1,1)".into()
        } else {
            String::new()
        }
    }
}
