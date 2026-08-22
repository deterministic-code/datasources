use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn mongodb_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("mongodb")
}

pub fn register_mongodb() {
    register(create_stub_datasource("mongodb"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_mongodb();
        let ds = create("mongodb", None).unwrap();
        assert_eq!(ds.provider(), "mongodb");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(mongodb_datasource().provider(), "mongodb");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("mongodb")), "mongodb");
    }
}
