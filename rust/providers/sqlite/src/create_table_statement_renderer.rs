use datasources::{Column, CreateTableStatement, CreateTableStatementTextRenderer, TableConstraint};
use std::collections::HashSet;

pub struct SqliteCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqliteCreateTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        crate::SqliteDriver::quote(name)
    }

    fn type_name(&self, column: &Column) -> String {
        crate::SqliteDriver::native_type(column)
    }

    fn column_default(&self, column: &Column) -> String {
        match &column.default {
            None => String::new(),
            Some(value) if value.name.is_none() => {
                format!(" DEFAULT {}", crate::SqliteDriver::native_default(column))
            }
            Some(value) => format!(
                " CONSTRAINT {} DEFAULT {}",
                self.quote(value.name.as_deref().unwrap_or("")),
                crate::SqliteDriver::native_default(column)
            ),
        }
    }

    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " PRIMARY KEY AUTOINCREMENT".into()
        } else {
            String::new()
        }
    }

    fn render(&self, statement: &CreateTableStatement) -> String {
        let identity = statement
            .columns
            .iter()
            .filter(|column| column.identity)
            .map(|column| column.name.as_str())
            .collect::<HashSet<_>>();
        let mut parts = statement
            .columns
            .iter()
            .map(|column| self.column(column))
            .collect::<Vec<_>>();
        parts.extend(
            statement
                .constraints
                .iter()
                .filter(|constraint| match constraint {
                    TableConstraint::PrimaryKey(pk) => {
                        !pk.columns.iter().all(|column| identity.contains(column.as_str()))
                    }
                    _ => true,
                })
                .map(|constraint| self.constraint(constraint)),
        );
        format!(
            "CREATE TABLE {} ({}){}",
            self.quote(&statement.table),
            parts.join(", "),
            self.suffix()
        )
    }
}
