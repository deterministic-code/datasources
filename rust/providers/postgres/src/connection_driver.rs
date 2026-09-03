use datasources::{DatasourceError, Result};
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

pub(crate) fn map_err(error: tokio_postgres::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

pub struct PostgresConnectionDriver {
    schema: Option<String>,
    pub(crate) client: Mutex<Option<tokio_postgres::Client>>,
}

impl PostgresConnectionDriver {
    pub fn new(schema: Option<String>) -> Self {
        Self {
            schema,
            client: Mutex::new(None),
        }
    }

    pub async fn open(&self, url: &str) -> Result<()> {
        let mut slot = self.client.lock().await;
        if slot.is_some() {
            return Ok(());
        }
        let (client, connection) = tokio_postgres::connect(url, NoTls).await.map_err(map_err)?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        if let Some(schema) = &self.schema {
            client
                .batch_execute(&format!("SET search_path TO {schema}"))
                .await
                .map_err(map_err)?;
        }
        *slot = Some(client);
        Ok(())
    }

    pub async fn close(&self) -> Result<()> {
        *self.client.lock().await = None;
        Ok(())
    }
}
