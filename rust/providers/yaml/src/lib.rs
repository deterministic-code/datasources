use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct YamlDatasource;

impl YamlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for YamlDatasource {
    fn provider(&self) -> &str {
        "yaml"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("yaml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(YamlDatasource::factory());
        assert_eq!(create("yaml", None).unwrap().provider(), "yaml");
    }
}
