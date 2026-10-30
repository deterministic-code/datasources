use crate::SqliteDriver;
use datasources::{
    CreateIndexStatement, DropIndexStatement, DropTableStatement, DropTriggerStatement,
    ForeignKeyStatement,
};

pub struct SqliteDropTableStatementRenderer;

impl SqliteDropTableStatementRenderer {
    pub fn render(&self, statement: &DropTableStatement) -> String {
        format!("DROP TABLE IF EXISTS {}", SqliteDriver::quote(statement.table()))
    }
}

pub struct SqliteCreateIndexStatementRenderer;

impl SqliteCreateIndexStatementRenderer {
    pub fn render(&self, statement: &CreateIndexStatement) -> String {
        format!(
            "CREATE {}INDEX {} ON {} ({})",
            if statement.unique() { "UNIQUE " } else { "" },
            SqliteDriver::quote(statement.name()),
            SqliteDriver::quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| SqliteDriver::quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub struct SqliteDropIndexStatementRenderer;

impl SqliteDropIndexStatementRenderer {
    pub fn render(&self, statement: &DropIndexStatement) -> String {
        format!("DROP INDEX IF EXISTS {}", SqliteDriver::quote(statement.name()))
    }
}

pub struct SqliteDropTriggerStatementRenderer;

impl SqliteDropTriggerStatementRenderer {
    pub fn render(&self, statement: &DropTriggerStatement) -> String {
        format!("DROP TRIGGER IF EXISTS {}", SqliteDriver::quote(statement.name()))
    }
}

pub struct SqliteForeignKeyStatementRenderer;

impl SqliteForeignKeyStatementRenderer {
    pub fn render(&self, statement: &ForeignKeyStatement) -> String {
        panic!(
            "{}",
            if statement.kind() == "drop" {
                "SQLite cannot DROP CONSTRAINT"
            } else {
                "SQLite cannot ADD CONSTRAINT"
            }
        )
    }
}
