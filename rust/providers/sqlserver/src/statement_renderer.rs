use crate::alter_table_statement_renderer::SqlServerAlterTableStatementRenderer;
use crate::stored_procedure_statement_renderer::SqlServerStoredProcedureStatementRenderer;
use crate::create_table_statement_renderer::SqlServerCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqlServerCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::SqlServerDropRoutineStatementRenderer;
use crate::identity_insert_statement_renderer::SqlServerIdentityInsertStatementRenderer;
use datasources::{
    AlterTableStatement, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateUpdateStoredProcedureStatement,
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    DropIndexStatement, DropRoutineStatement, IdentityInsertStatement, StatementTextRenderer,
    StatementVisitor,
};

pub struct SqlServerStatementRenderer;

impl StatementVisitor for SqlServerStatementRenderer {
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
}
