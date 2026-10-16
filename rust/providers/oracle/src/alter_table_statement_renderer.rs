use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer,
    CreateTableStatementTextRenderer,
};

pub struct OracleAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for OracleAlterTableStatementRenderer {}

impl AlterTableStatementTextRenderer for OracleAlterTableStatementRenderer {
    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AddColumn(column) => format!("ADD ({})", self.column(column)),
            AlterAction::AlterColumn(column) => format!("MODIFY ({})", self.column(column)),
            other => {
                struct DefaultAction;
                impl CreateTableStatementTextRenderer for DefaultAction {}
                impl AlterTableStatementTextRenderer for DefaultAction {}
                DefaultAction.action_sql(other)
            }
        }
    }
}

impl OracleAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}
