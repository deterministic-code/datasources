use datasources::{Column, CreateTableStatementTextRenderer, NotNull};

pub struct MysqlCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for MysqlCreateTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        crate::MysqlDriver::quote(name)
    }

    fn type_name(&self, column: &Column) -> String {
        crate::MysqlDriver::native_type(column)
    }

    fn column_default(&self, column: &Column) -> String {
        match column.default.as_ref() {
            None => String::new(),
            Some(_) => format!(" DEFAULT {}", crate::MysqlDriver::native_default(column)),
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
