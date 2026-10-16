use crate::alter_table_statement_renderer::OracleAlterTableStatementRenderer;
use crate::create_table_statement_renderer::OracleCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::OracleCreateTriggerStatementRenderer;
use crate::drop_table_statement_renderer::OracleDropTableStatementRenderer;
use datasources::{
    AlterTableStatement, CreateTableStatement,
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
}
