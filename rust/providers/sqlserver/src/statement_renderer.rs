use crate::alter_table_statement_renderer::SqlServerAlterTableStatementRenderer;
use crate::stored_procedure_statement_renderer::SqlServerStoredProcedureStatementRenderer;
use crate::create_table_statement_renderer::SqlServerCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqlServerCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::SqlServerDropRoutineStatementRenderer;
use crate::identity_insert_statement_renderer::SqlServerIdentityInsertStatementRenderer;
use crate::sql_seed_statement_renderer::SqlServerSqlSeedStatementRenderer;
use datasources::{
    AlterTableStatement, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement,
    CreateNestedReadStoredProcedureStatement, CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    CreateIndexStatement, DropIndexStatement, DropRoutineStatement, DropTableStatement,
    ForeignKeyStatement, IdentityInsertStatement, ProcedureSource, SqlSeedStatement,
    sql_procedure_statements_of, Statement, StatementTextRenderer, StatementVisitor,
};

pub struct SqlServerStatementRenderer;

impl StatementVisitor for SqlServerStatementRenderer {
    fn procedures(&self, source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
        sql_procedure_statements_of(source)
    }

    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerCreateTableStatementRenderer.render(statement)
    }

    fn visit_alter_table(
        &self,
        statement: &AlterTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerAlterTableStatementRenderer.render(statement)
    }

    fn visit_drop_index(
        &self,
        statement: &DropIndexStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!("DROP INDEX {} ON {}", statement.name(), statement.table())
    }

    fn visit_create_trigger(
        &self,
        statement: &CreateTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerCreateTriggerStatementRenderer.render(statement)
    }

    fn visit_identity_insert(
        &self,
        statement: &IdentityInsertStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerIdentityInsertStatementRenderer.render(statement)
    }

    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerDropRoutineStatementRenderer.render(statement)
    }

    fn visit_create_create_stored_procedure(&self, statement: &CreateCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_create(statement)
    }
    fn visit_create_read_stored_procedure(&self, statement: &CreateReadStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_read(statement)
    }
    fn visit_create_nested_read_stored_procedure(&self, statement: &CreateNestedReadStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_nested_read(statement)
    }
    fn visit_create_update_stored_procedure(&self, statement: &CreateUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_update(statement)
    }
    fn visit_create_delete_stored_procedure(&self, statement: &CreateDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_delete(statement)
    }
    fn visit_create_bulk_create_stored_procedure(&self, statement: &CreateBulkCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_bulk_create(statement)
    }
    fn visit_create_bulk_update_stored_procedure(&self, statement: &CreateBulkUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_bulk_update(statement)
    }
    fn visit_create_bulk_delete_stored_procedure(&self, statement: &CreateBulkDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        SqlServerStoredProcedureStatementRenderer.render_bulk_delete(statement)
    }

    fn visit_sql_seed(&self, statement: &SqlSeedStatement, _text: &StatementTextRenderer) -> String {
        SqlServerSqlSeedStatementRenderer.render(statement)
    }

    fn visit_drop_table(&self, statement: &DropTableStatement, _text: &StatementTextRenderer) -> String {
        format!("DROP TABLE IF EXISTS {}", crate::SqlServerDriver::quote(statement.table()))
    }

    fn visit_create_index(&self, statement: &CreateIndexStatement, _text: &StatementTextRenderer) -> String {
        format!(
            "CREATE {}INDEX {} ON {} ({})",
            if statement.unique() { "UNIQUE " } else { "" },
            crate::SqlServerDriver::quote(statement.name()),
            crate::SqlServerDriver::quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| crate::SqlServerDriver::quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn visit_foreign_key(&self, statement: &ForeignKeyStatement, _text: &StatementTextRenderer) -> String {
        let table = crate::SqlServerDriver::quote(statement.table());
        let name = crate::SqlServerDriver::quote(statement.name());
        if statement.kind() == "drop" {
            return format!("ALTER TABLE {table} DROP CONSTRAINT {name}");
        }
        let cols = statement
            .columns()
            .iter()
            .map(|column| crate::SqlServerDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        let refs = statement
            .references_columns()
            .iter()
            .map(|column| crate::SqlServerDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {} ({refs})",
            crate::SqlServerDriver::quote(statement.references_table())
        )
    }
}
