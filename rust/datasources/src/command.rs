use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use crate::statement::Statement;
use crate::statement_parser::StatementParser;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Command: Send + Sync {
    fn connection(&self) -> Arc<dyn Connection>;
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>);
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
}
