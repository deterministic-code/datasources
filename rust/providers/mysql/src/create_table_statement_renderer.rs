use crate::driver::column_sql;
use datasources::{Column, CreateTableStatementTextRenderer};

pub struct MysqlCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for MysqlCreateTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }

    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " AUTO_INCREMENT".into()
        } else {
            String::new()
        }
    }
}
