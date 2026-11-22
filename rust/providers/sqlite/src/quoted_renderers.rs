use crate::SqliteDriver;
use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqliteCreateTriggerStatementRenderer;
use datasources::{
    CreateIndexStatement, CreateTableStatement, CreateTableStatementTextRenderer,
    DropIndexStatement, DropTableStatement, DropTriggerStatement, ForeignKeyStatement, RebuildColumn,
    RebuildTableStatement,
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
        let columns = create
            .columns
            .iter()
            .map(|column| RebuildColumn::copied(&column.name, &column.name))
            .collect();
        SqliteRebuildTableStatementRenderer.render(&RebuildTableStatement::new(
            statement.table(),
            CreateTableStatement::new(&create.table, &create.columns, &create.constraints),
            columns,
            vec![],
            vec![],
        ))
    }
}

pub struct SqliteRebuildTableStatementRenderer;

impl SqliteRebuildTableStatementRenderer {
    pub fn render(&self, statement: &RebuildTableStatement) -> String {
        let dest = &statement.create().table;
        let tmp = format!("{dest}__new");
        let rebuilt = CreateTableStatement::new(&tmp, &statement.create().columns, &statement.create().constraints);
        let names = statement
            .columns()
            .iter()
            .map(|column| SqliteDriver::quote(&column.name))
            .collect::<Vec<_>>()
            .join(", ");
        let values = statement
            .columns()
            .iter()
            .map(select)
            .collect::<Vec<_>>()
            .join(", ");
        let from = SqliteDriver::quote(statement.table());
        let to = SqliteDriver::quote(dest);
        let tmp_name = SqliteDriver::quote(&tmp);
        let mut parts = vec![
            SqliteCreateTableStatementRenderer.render(&rebuilt),
            format!("INSERT INTO {tmp_name} ({names}) SELECT {values} FROM {from}"),
            SqliteDropTableStatementRenderer.render(&DropTableStatement::new(statement.table())),
            format!("ALTER TABLE {tmp_name} RENAME TO {to}"),
        ];
        parts.extend(statement.indexes().iter().map(|index| SqliteCreateIndexStatementRenderer.render(index)));
        parts.extend(statement.triggers().iter().map(|trigger| SqliteCreateTriggerStatementRenderer.render(trigger)));
        parts.into_iter().filter(|sql| !sql.is_empty()).collect::<Vec<_>>().join(";\n")
    }
}

fn select(column: &RebuildColumn) -> String {
    match (&column.source, &column.cast, &column.literal) {
        (Some(source), Some(cast), _) => format!("CAST({} AS {cast})", SqliteDriver::quote(source)),
        (Some(source), None, _) => SqliteDriver::quote(source),
        (None, _, Some(literal)) => literal.clone(),
        (None, _, None) => "NULL".into(),
    }
}
