use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct OracleDatasource;

impl OracleDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for OracleDatasource {
    fn provider(&self) -> &str {
        "oracle"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("oracle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(OracleDatasource::factory());
        assert_eq!(create("oracle", None).unwrap().provider(), "oracle");
    }
}
