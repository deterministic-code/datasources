use crate::driver::column_sql;
use datasources::{Column, CreateTableStatementTextRenderer};

pub struct SqliteCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqliteCreateTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }
}
