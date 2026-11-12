use crate::connection::MemoryConnection;
use crate::statement_renderer::MemoryStatementRenderer;
use async_trait::async_trait;
use datasources::{
    ArrayReader, Command, CommandMiddleware, CommandMiddlewareHost, Native, Parameter,
    Reader, Response, Result, Statement, StatementTextRenderer,
};
use std::sync::Arc;

pub struct MemoryCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MemoryConnection>,
    statements: StatementTextRenderer,
}

impl MemoryCommand {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
            statements: StatementTextRenderer::new(Some(Box::new(MemoryStatementRenderer)), None),
        }
    }
}

#[async_trait]
impl Command for MemoryCommand {
    fn connection(&self) -> Arc<dyn datasources::Connection> {
        Arc::clone(&self.connection) as Arc<dyn datasources::Connection>
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    fn schema(&self) -> Option<datasources::Schema> {
        self.host.schema()
    }

    fn set_schema(&self, schema: Option<datasources::Schema>) {
        self.host.set_schema(schema);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.execute_reader(text.text(), parameters).await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        self.execute(text.text(), parameters).await
    }

    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    connection.state.engine.execute(&query, params.as_deref())?;
                    let rows = connection.state.engine.records();
                    Ok((
                        Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                        rows,
                    ))
                }
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        if text.trim().is_empty() {
            return Err(datasources::DatasourceError::message("empty command"));
        }
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let response = connection.state.engine.execute(&query, params.as_deref())?;
                    Ok((response, vec![]))
                }
            })
            .await
    }

    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let text = self.statements.render(statement);
        self.execute_reader(&text, parameters).await
    }

    async fn execute_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let text = self.statements.render(statement);
        self.execute(&text, parameters).await
    }
}
