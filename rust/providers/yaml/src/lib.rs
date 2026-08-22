use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn yaml_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("yaml")
}

pub fn register_yaml() {
    register(create_stub_datasource("yaml"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_yaml();
        let ds = create("yaml", None).unwrap();
        assert_eq!(ds.provider(), "yaml");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(yaml_datasource().provider(), "yaml");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("yaml")), "yaml");
    }
}
