use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use super::table_constraint::TableConstraint;
use crate::builders::Column;

pub enum AlterAction {
    AddColumn(Column),
    DropColumn { name: String },
    AlterColumn(Column),
    RenameColumn { from: String, to: String },
    RenameTable { to: String },
    AddConstraint(TableConstraint),
    DropConstraint { name: String },
}

pub struct AlterTableStatement {
    table: String,
    action: AlterAction,
}

impl AlterTableStatement {
    pub fn new(table: impl AsRef<str>, action: AlterAction) -> Self {
        Self {
            table: table.as_ref().to_string(),
            action,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn action(&self) -> &AlterAction {
        &self.action
    }
}

impl Statement for AlterTableStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_alter_table(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for AlterTableStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
