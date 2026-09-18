use crate::connection::MysqlConnection;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, ArrayReader, Command, CommandMiddleware, CommandMiddlewareHost, Parameter,
    Reader, Response, Result,
};
use std::sync::Arc;

pub struct MysqlCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MysqlConnection>,
}

impl MysqlCommand {
    pub fn new(connection: Arc<MysqlConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }
}

#[async_trait]
impl Command for MysqlCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mapped = parameters_to_map(params.as_deref());
                    let rows = connection.driver.query(&sql, &mapped).await?;
                    Ok((
                        Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                        rows,
                    ))
                }
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |sql, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mapped = parameters_to_map(params.as_deref());
                    let result = connection.driver.execute(&sql, &mapped).await?;
                    Ok((Response::new(result.result, result.rows_affected), vec![]))
                }
            })
            .await
    }
}
