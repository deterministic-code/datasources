use crate::datasource::Datasource;
use std::sync::Arc;

pub trait DatasourceFactory: Send + Sync {
    fn provider(&self) -> &str;
    fn create(&self, options: Option<&str>) -> Arc<dyn Datasource>;
}
