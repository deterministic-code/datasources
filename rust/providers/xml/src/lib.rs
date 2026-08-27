use datasources::{create_stub_datasource, Datasource, DatasourceFactory};
use std::sync::Arc;

pub struct XmlDatasource;

impl XmlDatasource {
    pub fn factory() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DatasourceFactory for XmlDatasource {
    fn provider(&self) -> &str {
        "xml"
    }
    fn create(&self, _options: Option<&str>) -> Arc<dyn Datasource> {
        create_stub_datasource("xml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset};

    #[test]
    fn creates() {
        reset();
        register(XmlDatasource::factory());
        assert_eq!(create("xml", None).unwrap().provider(), "xml");
    }
}
