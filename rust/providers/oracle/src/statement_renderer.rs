use crate::alter_table_statement_renderer::OracleAlterTableStatementRenderer;
use crate::stored_procedure_statement_renderer::OracleStoredProcedureStatementRenderer;
use crate::create_table_statement_renderer::OracleCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::OracleCreateTriggerStatementRenderer;
use crate::drop_table_statement_renderer::OracleDropTableStatementRenderer;
use datasources::{
    AlterTableStatement, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement,
    CreateReadStoredProcedureStatement, CreateTableStatement, CreateUpdateStoredProcedureStatement,
    CreateTableStatementTextRenderer, CreateTriggerStatement, DropIndexStatement,
    DropTableStatement, DropTriggerStatement, StatementTextRenderer, StatementVisitor,
};

pub struct OracleStatementRenderer;

impl StatementVisitor for OracleStatementRenderer {
    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        OracleCreateTableStatementRenderer.render(statement)
    }

    fn visit_alter_table(
        &self,
        statement: &AlterTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        OracleAlterTableStatementRenderer.render(statement)
    }

    fn visit_drop_index(
        &self,
        statement: &DropIndexStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!("DROP INDEX {}", statement.name())
    }

    fn visit_drop_trigger(
        &self,
        statement: &DropTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!("DROP TRIGGER {}", statement.name())
    }

    fn visit_create_trigger(
        &self,
        statement: &CreateTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        OracleCreateTriggerStatementRenderer.render(statement)
    }

    fn visit_drop_table(
        &self,
        statement: &DropTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        OracleDropTableStatementRenderer.render(statement)
    }

    fn visit_create_create_stored_procedure(&self, statement: &CreateCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_create(statement)
    }
    fn visit_create_read_stored_procedure(&self, statement: &CreateReadStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_read(statement)
    }
    fn visit_create_update_stored_procedure(&self, statement: &CreateUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_update(statement)
    }
    fn visit_create_delete_stored_procedure(&self, statement: &CreateDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_delete(statement)
    }
    fn visit_create_bulk_create_stored_procedure(&self, statement: &CreateBulkCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_bulk_create(statement)
    }
    fn visit_create_bulk_update_stored_procedure(&self, statement: &CreateBulkUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_bulk_update(statement)
    }
    fn visit_create_bulk_delete_stored_procedure(&self, statement: &CreateBulkDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        OracleStoredProcedureStatementRenderer.render_bulk_delete(statement)
    }
}
