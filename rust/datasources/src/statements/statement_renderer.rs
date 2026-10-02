use crate::select_statement::SelectStatement;
use super::create_index_statement::CreateIndexStatement;
use super::create_index_statement_text_renderer::CreateIndexStatementTextRenderer;
use super::create_table_statement::CreateTableStatement;
use super::create_table_statement_text_renderer;
use super::delete_statement::DeleteStatement;
use super::delete_statement_text_renderer::DeleteStatementTextRenderer;
use super::execute_stored_procedure_statement::ExecuteStoredProcedureStatement;
use super::execute_stored_procedure_statement_text_renderer::ExecuteStoredProcedureStatementTextRenderer;
use super::insert_statement::InsertStatement;
use super::insert_statement_text_renderer::InsertStatementTextRenderer;
use super::return_statement::ReturnStatement;
use super::return_statement_text_renderer::ReturnStatementTextRenderer;
use super::select_inserted_id_statement::SelectInsertedIdStatement;
use super::select_inserted_id_statement_text_renderer::SelectInsertedIdStatementTextRenderer;
use super::select_statement_text_renderer::SelectStatementTextRenderer;
use super::statement_text_renderer::StatementTextRenderer;
use super::update_statement::UpdateStatement;
use super::update_statement_text_renderer::UpdateStatementTextRenderer;

pub trait StatementVisitor: Send + Sync {
    fn visit_select(&self, statement: &SelectStatement, text: &StatementTextRenderer) -> String {
        SelectStatementTextRenderer::new(text).render(statement)
    }

    fn visit_insert(&self, statement: &InsertStatement, _text: &StatementTextRenderer) -> String {
        InsertStatementTextRenderer.render(statement)
    }

    fn visit_update(&self, statement: &UpdateStatement, text: &StatementTextRenderer) -> String {
        UpdateStatementTextRenderer::new(text).render(statement)
    }

    fn visit_delete(&self, statement: &DeleteStatement, text: &StatementTextRenderer) -> String {
        DeleteStatementTextRenderer::new(text).render(statement)
    }

    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        create_table_statement_text_renderer::render(statement)
    }

    fn visit_create_index(
        &self,
        statement: &CreateIndexStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        CreateIndexStatementTextRenderer.render(statement)
    }

    fn visit_execute_stored_procedure(
        &self,
        statement: &ExecuteStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        ExecuteStoredProcedureStatementTextRenderer.render(statement)
    }

    fn visit_return(&self, statement: &ReturnStatement, _text: &StatementTextRenderer) -> String {
        ReturnStatementTextRenderer.render(statement)
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SelectInsertedIdStatementTextRenderer.render(statement)
    }
}

pub struct StatementRenderer;

impl StatementVisitor for StatementRenderer {}
