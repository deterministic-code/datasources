use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct TextDatasource;

impl TextDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for TextDatasource {
    fn provider(&self) -> &str {
        "text"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("text")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(TextDatasource::factory());
        assert_eq!(create("text", None).unwrap().provider(), "text");
    }
}
