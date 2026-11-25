use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::MysqlCreateTriggerStatementRenderer;
use crate::MysqlDriver;
use datasources::{
    CreateIndexStatement, CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    RebuildColumn, RebuildTableStatement,
};

pub struct MysqlRebuildTableStatementRenderer;

impl MysqlRebuildTableStatementRenderer {
    pub fn render(&self, statement: &RebuildTableStatement) -> String {
        let dest = &statement.create().table;
        let tmp = format!("{dest}__new");
        let old = format!("{}__old", statement.table());
        let rebuilt = CreateTableStatement::new(&tmp, &statement.create().columns, &statement.create().constraints);
        let names = statement
            .columns()
            .iter()
            .map(|column| MysqlDriver::quote(&column.name))
            .collect::<Vec<_>>()
            .join(", ");
        let values = statement
            .columns()
            .iter()
            .map(select)
            .collect::<Vec<_>>()
            .join(", ");
        let from = MysqlDriver::quote(statement.table());
        let to = MysqlDriver::quote(dest);
        let tmp_name = MysqlDriver::quote(&tmp);
        let old_name = MysqlDriver::quote(&old);
        let mut parts = vec![
            "SET FOREIGN_KEY_CHECKS = 0".into(),
            MysqlCreateTableStatementRenderer.render(&rebuilt),
            format!("INSERT INTO {tmp_name} ({names}) SELECT {values} FROM {from}"),
        ];
        for index in statement.indexes() {
            parts.push(index_sql(index, &tmp));
        }
        for trigger in statement.triggers() {
            parts.push(MysqlCreateTriggerStatementRenderer.render(&CreateTriggerStatement::new(
                &tmp,
                trigger.name(),
                trigger.updated_column(),
                trigger.key_column(),
            )));
        }
        parts.push(format!("RENAME TABLE {from} TO {old_name}, {tmp_name} TO {to}"));
        parts.push(format!("DROP TABLE IF EXISTS {old_name}"));
        parts.push("SET FOREIGN_KEY_CHECKS = 1".into());
        parts.into_iter().filter(|sql| !sql.is_empty()).collect::<Vec<_>>().join(";\n")
    }
}

fn select(column: &RebuildColumn) -> String {
    match (&column.source, &column.cast, &column.literal) {
        (Some(source), Some(cast), _) => {
            format!("CAST({} AS {})", MysqlDriver::quote(source), cast_target(cast))
        }
        (Some(source), None, _) => MysqlDriver::quote(source),
        (None, _, Some(literal)) => literal.clone(),
        (None, _, None) => "NULL".into(),
    }
}

fn cast_target(type_name: &str) -> &'static str {
    let native = type_name.to_ascii_uppercase();
    if native.contains("INT") || native.contains("BOOL") {
        "SIGNED"
    } else if native.contains("CHAR") || native.contains("TEXT") || native.contains("UUID") {
        "CHAR"
    } else if native.contains("DATE") || native.contains("TIME") {
        "DATETIME"
    } else if native.contains("BLOB") || native.contains("BINARY") {
        "BINARY"
    } else if native.contains("DEC")
        || native.contains("NUM")
        || native.contains("FLOAT")
        || native.contains("DOUBLE")
        || native.contains("REAL")
    {
        "DECIMAL"
    } else {
        "CHAR"
    }
}

fn index_sql(index: &CreateIndexStatement, table: &str) -> String {
    let cols: Vec<&str> = index.columns().iter().map(String::as_str).collect();
    let retargeted = CreateIndexStatement::new(index.name(), table, &cols, index.unique());
    format!(
        "CREATE {}INDEX {} ON {} ({})",
        if retargeted.unique() { "UNIQUE " } else { "" },
        MysqlDriver::quote(&crate::identifier::fit(retargeted.name())),
        MysqlDriver::quote(retargeted.table()),
        retargeted
            .columns()
            .iter()
            .map(|column| MysqlDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", "),
    )
}
