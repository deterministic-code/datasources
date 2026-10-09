use crate::driver::column_sql;
use datasources::{Column, CreateTableStatement, CreateTableStatementTextRenderer, TableConstraint};
use std::collections::HashSet;

pub struct SqliteCreateTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqliteCreateTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
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
