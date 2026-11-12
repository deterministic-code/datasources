use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::datasource_error::DatasourceError;
use crate::native::Native;
use crate::schema::Schema;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use crate::statement_parser::StatementParser;
use crate::{ReturnStatement, Statement, StatementKind, Value};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Command: Send + Sync {
    fn connection(&self) -> Arc<dyn Connection>;
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>);
    fn schema(&self) -> Option<Schema> {
        None
    }
    fn set_schema(&self, _schema: Option<Schema>) {}
    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>>;
    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response>;
    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        if text.trim().is_empty() {
            return Err(crate::DatasourceError::message("empty command"));
        }
        let statement = StatementParser::parse(text)?;
        self.execute_reader_statement(statement.as_ref(), parameters)
            .await
    }
    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        if text.trim().is_empty() {
            return Err(crate::DatasourceError::message("empty command"));
        }
        let statement = StatementParser::parse(text)?;
        self.execute_statement(statement.as_ref(), parameters).await
    }
    async fn execute_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        self.execute_native(
            &Native::new(crate::DefaultStatementTextRenderer::instance().render(statement)),
            parameters,
        )
        .await
    }
    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.execute_reader_native(
            &Native::new(crate::DefaultStatementTextRenderer::instance().render(statement)),
            parameters,
        )
        .await
    }

    fn reads_cell(&self, statement: &dyn Statement) -> bool {
        statement.kind() == StatementKind::Select || statement.as_any().is::<ReturnStatement>()
    }

    async fn first_cell(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let reader = self.execute_reader_statement(statement, parameters).await?;
        let response = if !reader.next().await? {
            Ok(Response::new(
                Value::Null,
                0,
                parameters.map(|list| list.to_vec()),
            ))
        } else {
            let row = reader
                .current()
                .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
            let cell = row.into_values().next().unwrap_or(Value::Null);
            Ok(Response::new(cell, 1, parameters.map(|list| list.to_vec())))
        };
        let _ = reader.close().await;
        response
    }
}
