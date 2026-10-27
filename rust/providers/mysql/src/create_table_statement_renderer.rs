use crate::driver::column_sql;
use datasources::{Column, ColumnDefault, CreateTableStatementTextRenderer, NotNull};

pub struct MysqlCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for MysqlCreateTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }

    fn column_default(&self, column: &Column) -> String {
        match column.default.as_ref() {
            None => String::new(),
            Some(ColumnDefault { value, .. }) => format!(" DEFAULT {value}"),
        }
    }

    fn not_null_sql(&self, value: Option<&NotNull>) -> String {
        match value {
            None => String::new(),
            Some(_) => " NOT NULL".into(),
        }
    }

    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " AUTO_INCREMENT".into()
        } else {
            String::new()
        }
    }
}
