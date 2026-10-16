use crate::driver::column_sql;
use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column,
    CreateTableStatementTextRenderer,
};

pub struct MysqlAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for MysqlAlterTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }

    fn identity(&self, column: &Column) -> String {
        if column.identity {
            " AUTO_INCREMENT".into()
        } else {
            String::new()
        }
    }
}

impl AlterTableStatementTextRenderer for MysqlAlterTableStatementRenderer {
    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AlterColumn(column) => format!("MODIFY COLUMN {}", self.column(column)),
            other => {
                struct DefaultAction;
                impl CreateTableStatementTextRenderer for DefaultAction {}
                impl AlterTableStatementTextRenderer for DefaultAction {}
                DefaultAction.action_sql(other)
            }
        }
    }
}

impl MysqlAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}
