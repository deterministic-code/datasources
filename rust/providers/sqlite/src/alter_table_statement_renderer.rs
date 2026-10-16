use crate::driver::column_sql;
use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column,
    CreateTableStatementTextRenderer,
};

pub struct SqliteAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqliteAlterTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }
}

impl AlterTableStatementTextRenderer for SqliteAlterTableStatementRenderer {
    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AlterColumn(_) => panic!("SQLite cannot ALTER COLUMN"),
            AlterAction::AddConstraint(_) => panic!("SQLite cannot ADD CONSTRAINT"),
            AlterAction::DropConstraint { .. } => panic!("SQLite cannot DROP CONSTRAINT"),
            AlterAction::AddColumn(column) => format!("ADD COLUMN {}", self.column(column)),
            AlterAction::DropColumn { name } => format!("DROP COLUMN {}", self.quote(name)),
            AlterAction::RenameColumn { from, to } => {
                format!("RENAME COLUMN {} TO {}", self.quote(from), self.quote(to))
            }
            AlterAction::RenameTable { to } => format!("RENAME TO {}", self.quote(to)),
        }
    }
}

impl SqliteAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}
