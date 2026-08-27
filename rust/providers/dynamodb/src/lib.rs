use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct DynamoDbDatasource;

impl DynamoDbDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for DynamoDbDatasource {
    fn provider(&self) -> &str {
        "dynamodb"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("dynamodb")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(DynamoDbDatasource::factory());
        assert_eq!(create("dynamodb", None).unwrap().provider(), "dynamodb");
    }
}
