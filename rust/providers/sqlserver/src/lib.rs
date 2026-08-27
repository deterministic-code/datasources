use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct SqlServerDatasource;

impl SqlServerDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for SqlServerDatasource {
    fn provider(&self) -> &str {
        "sqlserver"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("sqlserver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(SqlServerDatasource::factory());
        assert_eq!(create("sqlserver", None).unwrap().provider(), "sqlserver");
    }
}
