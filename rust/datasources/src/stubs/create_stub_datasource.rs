use super::stub_datasource::StubDatasource;
use std::sync::Arc;

pub fn create_stub_datasource(provider: impl Into<String>) -> Arc<StubDatasource> {
    Arc::new(StubDatasource::new(provider))
}
