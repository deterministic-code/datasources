use crate::datasource_error::DatasourceError;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait CommandMiddleware: Send + Sync {
    async fn before_query(
        &self,
        query: &str,
        params: Option<&[Parameter]>,
    ) -> Result<Option<(String, Option<Vec<Parameter>>)>>;

    async fn after_query(
        &self,
        query: &str,
        params: Option<&[Parameter]>,
        results: &[Row],
        elapsed_ms: f64,
        error: Option<&DatasourceError>,
    ) -> Result<()>;
}
