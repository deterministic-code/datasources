use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column,
    CreateTableStatementTextRenderer, NotNull,
};

pub struct MysqlAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for MysqlAlterTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        crate::MysqlDriver::quote(name)
    }

    fn type_name(&self, column: &Column) -> String {
        crate::MysqlDriver::native_type(column)
    }

    fn column_default(&self, column: &Column) -> String {
        match column.default.as_ref() {
            None => String::new(),
            Some(_) => format!(" DEFAULT {}", crate::MysqlDriver::native_default(column)),
        }
    }

    fn not_null_sql(&self, value: Option<&NotNull>) -> String {
        match value {
            None => String::new(),
            Some(_) => " NOT NULL".into(),
        }
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
