use datasources::{Column, CreateTableStatementTextRenderer};

pub struct SqlServerCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqlServerCreateTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        crate::SqlServerDriver::quote(name)
    }

    fn type_name(&self, column: &Column) -> String {
        crate::SqlServerDriver::native_type(column)
    }

    fn column_default(&self, column: &Column) -> String {
        match &column.default {
            None => String::new(),
            Some(value) if value.name.is_none() => {
                format!(" DEFAULT {}", crate::SqlServerDriver::native_default(column))
            }
            Some(value) => format!(
                " CONSTRAINT {} DEFAULT {}",
                self.quote(value.name.as_deref().unwrap_or("")),
                crate::SqlServerDriver::native_default(column)
            ),
        }
    }

    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " IDENTITY(1,1)".into()
        } else {
            String::new()
        }
    }
}
