use crate::SqliteDriver;
use crate::quoted_renderers::{SqliteCreateIndexStatementRenderer, SqliteDropIndexStatementRenderer};
use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column, CreateIndexStatement,
    CreateTableStatementTextRenderer, DropIndexStatement, TableConstraint,
};

pub struct SqliteAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqliteAlterTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        SqliteDriver::quote(name)
    }

    fn type_name(&self, column: &Column) -> String {
        SqliteDriver::native_type(column)
    }

    fn column_default(&self, column: &Column) -> String {
        match &column.default {
            None => String::new(),
            Some(value) if value.name.is_none() => format!(" DEFAULT {}", SqliteDriver::native_default(column)),
            Some(value) => format!(
                " CONSTRAINT {} DEFAULT {}",
                self.quote(value.name.as_deref().unwrap_or("")),
                SqliteDriver::native_default(column)
            ),
        }
    }
}

impl AlterTableStatementTextRenderer for SqliteAlterTableStatementRenderer {
    fn render_alter(&self, statement: &AlterTableStatement) -> String {
        match statement.action() {
            AlterAction::AddConstraint(TableConstraint::Unique(unique)) => {
                let columns = unique.columns.iter().map(String::as_str).collect::<Vec<_>>();
                SqliteCreateIndexStatementRenderer.render(&CreateIndexStatement::new(
                    &unique.name,
                    statement.table(),
                    &columns,
                    true,
                ))
            }
            AlterAction::DropConstraint { name } => {
                SqliteDropIndexStatementRenderer.render(&DropIndexStatement::new(name, statement.table()))
            }
            _ => format!(
                "ALTER TABLE {} {}",
                self.quote(statement.table()),
                self.action_sql(statement.action())
            ),
        }
    }

    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AlterColumn(_) => panic!("SQLite cannot ALTER COLUMN"),
            AlterAction::AddConstraint(_) => panic!("SQLite cannot ADD CONSTRAINT"),
            AlterAction::AddColumn(column) => format!("ADD COLUMN {}", self.column(column)),
            AlterAction::DropColumn { name } => format!("DROP COLUMN {}", self.quote(name)),
            AlterAction::RenameColumn { from, to } => {
                format!("RENAME COLUMN {} TO {}", self.quote(from), self.quote(to))
            }
            AlterAction::RenameTable { to } => format!("RENAME TO {}", self.quote(to)),
            AlterAction::DropConstraint { .. } => panic!("SQLite cannot DROP CONSTRAINT"),
        }
    }
}

impl SqliteAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}
