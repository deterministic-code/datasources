use super::alter_table_statement::{AlterAction, AlterTableStatement};
use super::create_table_statement_text_renderer::CreateTableStatementTextRenderer;
use crate::builders::Column;
use crate::statements::not_null::NotNull;

pub trait AlterTableStatementTextRenderer: CreateTableStatementTextRenderer {
    fn render_alter(&self, statement: &AlterTableStatement) -> String {
        format!(
            "ALTER TABLE {} {}",
            self.quote(statement.table()),
            self.action_sql(statement.action())
        )
    }

    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AddColumn(column) => format!("ADD COLUMN {}", self.column(column)),
            AlterAction::DropColumn { name } => format!("DROP COLUMN {}", self.quote(name)),
            AlterAction::AlterColumn(column) => self.alter_column(column),
            AlterAction::RenameColumn { from, to } => {
                format!("RENAME COLUMN {} TO {}", self.quote(from), self.quote(to))
            }
            AlterAction::RenameTable { to } => format!("RENAME TO {}", self.quote(to)),
            AlterAction::AddConstraint(constraint) => format!("ADD {}", self.constraint(constraint)),
            AlterAction::DropConstraint { name } => {
                format!("DROP CONSTRAINT {}", self.quote(name))
            }
        }
    }

    fn alter_column(&self, column: &Column) -> String {
        format!(
            "ALTER COLUMN {} TYPE {}{}{}",
            self.quote(&column.name),
            self.type_name(column),
            self.column_default(column),
            not_null_clause(self, column.not_null.as_ref())
        )
    }
}

struct DefaultRenderer;

impl CreateTableStatementTextRenderer for DefaultRenderer {}
impl AlterTableStatementTextRenderer for DefaultRenderer {}

pub(crate) fn render(statement: &AlterTableStatement) -> String {
    DefaultRenderer.render_alter(statement)
}

fn not_null_clause<T: CreateTableStatementTextRenderer + ?Sized>(
    sql: &T,
    value: Option<&NotNull>,
) -> String {
    match value {
        None => String::new(),
        Some(NotNull { name: None }) => " NOT NULL".into(),
        Some(NotNull { name: Some(name) }) => format!(" CONSTRAINT {} NOT NULL", sql.quote(name)),
    }
}
