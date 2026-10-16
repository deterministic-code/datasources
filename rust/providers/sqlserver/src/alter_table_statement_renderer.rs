use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column,
    CreateTableStatementTextRenderer,
};

pub struct SqlServerAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for SqlServerAlterTableStatementRenderer {}

impl AlterTableStatementTextRenderer for SqlServerAlterTableStatementRenderer {
    fn render_alter(&self, statement: &AlterTableStatement) -> String {
        match statement.action() {
            AlterAction::RenameColumn { from, to } => {
                format!(
                    "EXEC sp_rename '{}.{}', '{to}', 'COLUMN'",
                    statement.table(),
                    from
                )
            }
            AlterAction::RenameTable { to } => {
                format!("EXEC sp_rename '{}', '{to}'", statement.table())
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
            AlterAction::AlterColumn(column) => format!(
                "ALTER COLUMN {} {}{}",
                self.quote(&column.name),
                self.type_name(column),
                not_null(column)
            ),
            other => {
                struct DefaultAction;
                impl CreateTableStatementTextRenderer for DefaultAction {}
                impl AlterTableStatementTextRenderer for DefaultAction {}
                DefaultAction.action_sql(other)
            }
        }
    }
}

impl SqlServerAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}

fn not_null(column: &Column) -> &'static str {
    match column.not_null {
        Some(_) => " NOT NULL",
        None => "",
    }
}
