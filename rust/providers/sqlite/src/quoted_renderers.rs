use crate::SqliteDriver;
use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use datasources::{
    CreateIndexStatement, CreateTableStatement, CreateTableStatementTextRenderer, DropIndexStatement,
    DropTableStatement, DropTriggerStatement, ForeignKeyStatement,
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
        if statement.kind() == "drop" {
            return String::new();
        }
        let Some(create) = statement.create() else {
            panic!("SQLite cannot ADD CONSTRAINT")
        };
        let tmp = format!("{}__new", statement.table());
        let rebuilt = CreateTableStatement::new(&tmp, &create.columns, &create.constraints);
        let cols = create
            .columns
            .iter()
            .map(|column| SqliteDriver::quote(&column.name))
            .collect::<Vec<_>>()
            .join(", ");
        let from = SqliteDriver::quote(statement.table());
        let to = SqliteDriver::quote(&tmp);
        [
            SqliteCreateTableStatementRenderer.render(&rebuilt),
            format!("INSERT INTO {to} ({cols}) SELECT {cols} FROM {from}"),
            SqliteDropTableStatementRenderer.render(&DropTableStatement::new(statement.table())),
            format!("ALTER TABLE {to} RENAME TO {from}"),
        ]
        .join(";\n")
    }
}
