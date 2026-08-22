use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn xml_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("xml")
}

pub fn register_xml() {
    register(create_stub_datasource("xml"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_xml();
        let ds = create("xml", None).unwrap();
        assert_eq!(ds.provider(), "xml");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(xml_datasource().provider(), "xml");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("xml")), "xml");
    }
}
