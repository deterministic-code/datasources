use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct MongoDbDatasource;

impl MongoDbDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for MongoDbDatasource {
    fn provider(&self) -> &str {
        "mongodb"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("mongodb")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(MongoDbDatasource::factory());
        assert_eq!(create("mongodb", None).unwrap().provider(), "mongodb");
    }
}
