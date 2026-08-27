use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct MysqlDatasource;

impl MysqlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for MysqlDatasource {
    fn provider(&self) -> &str {
        "mysql"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("mysql")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(MysqlDatasource::factory());
        assert_eq!(create("mysql", None).unwrap().provider(), "mysql");
    }
}
