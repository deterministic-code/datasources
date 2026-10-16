use crate::alter_table_statement_renderer::SqlServerAlterTableStatementRenderer;
use crate::create_routine_statement_renderer::SqlServerCreateRoutineStatementRenderer;
use crate::create_table_statement_renderer::SqlServerCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqlServerCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::SqlServerDropRoutineStatementRenderer;
use crate::identity_insert_statement_renderer::SqlServerIdentityInsertStatementRenderer;
use datasources::{
    AlterTableStatement, CreateRoutineStatement,
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

    fn visit_create_routine(
        &self,
        statement: &CreateRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerCreateRoutineStatementRenderer.render(statement)
    }

    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqlServerDropRoutineStatementRenderer.render(statement)
    }
}
