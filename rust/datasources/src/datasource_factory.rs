use crate::datasource::Datasource;
use std::collections::HashMap;
use std::sync::Arc;

pub trait DatasourceFactory: Send + Sync {
    fn provider(&self) -> &str;
    fn create(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Datasource>;
}
