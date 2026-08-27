use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct PostgresDatasource;

impl PostgresDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for PostgresDatasource {
    fn provider(&self) -> &str {
        "postgres"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("postgres")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(PostgresDatasource::factory());
        assert_eq!(create("postgres", None).unwrap().provider(), "postgres");
    }
}
