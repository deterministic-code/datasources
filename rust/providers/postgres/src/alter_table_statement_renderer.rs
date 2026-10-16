use crate::driver::column_sql;
use datasources::{
    AlterAction, AlterTableStatement, AlterTableStatementTextRenderer, Column,
    CreateTableStatementTextRenderer,
};

pub struct PostgresAlterTableStatementRenderer;

impl CreateTableStatementTextRenderer for PostgresAlterTableStatementRenderer {
    fn type_name(&self, column: &Column) -> String {
        column_sql(&column.type_name)
            .map(str::to_string)
            .unwrap_or_else(|| column.type_name.clone())
    }
}

impl AlterTableStatementTextRenderer for PostgresAlterTableStatementRenderer {
    fn action_sql(&self, action: &AlterAction) -> String {
        match action {
            AlterAction::AlterColumn(column) => postgres_alter(self, column),
            other => {
                struct DefaultAction;
                impl CreateTableStatementTextRenderer for DefaultAction {}
                impl AlterTableStatementTextRenderer for DefaultAction {}
                DefaultAction.action_sql(other)
            }
        }
    }
}

impl PostgresAlterTableStatementRenderer {
    pub fn render(&self, statement: &AlterTableStatement) -> String {
        self.render_alter(statement)
    }
}

fn postgres_alter(sql: &PostgresAlterTableStatementRenderer, column: &Column) -> String {
    let name = sql.quote(&column.name);
    let default_sql = match &column.default {
        Some(value) => format!("ALTER COLUMN {name} SET DEFAULT {}", value.value),
        None => format!("ALTER COLUMN {name} DROP DEFAULT"),
    };
    let null_sql = match &column.not_null {
        Some(_) => format!("ALTER COLUMN {name} SET NOT NULL"),
        None => format!("ALTER COLUMN {name} DROP NOT NULL"),
    };
    format!(
        "ALTER COLUMN {name} TYPE {}, {default_sql}, {null_sql}",
        sql.type_name(column)
    )
}
