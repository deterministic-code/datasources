use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn oracle_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("oracle")
}

pub fn register_oracle() {
    register(create_stub_datasource("oracle"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_oracle();
        let ds = create("oracle", None).unwrap();
        assert_eq!(ds.provider(), "oracle");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(oracle_datasource().provider(), "oracle");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("oracle")), "oracle");
    }
}
