use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::{
    parameters_to_map, ArrayReader, Command, MiddlewareAware, MiddlewareHook, MiddlewareHost,
    Parameter, Reader, Response, Result,
};
use std::sync::Arc;

pub struct SqliteCommand {
    host: MiddlewareHost,
    connection: Arc<SqliteConnection>,
}

impl SqliteCommand {
    pub fn new(connection: Arc<SqliteConnection>) -> Self {
        Self {
            host: MiddlewareHost::new(),
            connection,
        }
    }
}

impl MiddlewareAware for SqliteCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for SqliteCommand {
    async fn execute_reader(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let sql = text.unwrap_or("").to_string();
        let params = parameters_to_map(parameters);
        self.host
            .run("executeReader", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let rows = connection.driver.query(&sql, &params).await?;
                    Ok(Box::new(ArrayReader::new(rows)) as Box<dyn Reader>)
                }
            })
            .await
    }

    async fn execute(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        let connection = self.connection.clone();
        let sql = text.unwrap_or("").to_string();
        let params = parameters_to_map(parameters);
        self.host
            .run("execute", extras, move || {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let result = connection.driver.execute(&sql, &params).await?;
                    Ok(Response::new(result.result, result.rows_affected))
                }
            })
            .await
    }
}
