use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn postgres_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("postgres")
}

pub fn register_postgres() {
    register(create_stub_datasource("postgres"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_postgres();
        let ds = create("postgres", None).unwrap();
        assert_eq!(ds.provider(), "postgres");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(postgres_datasource().provider(), "postgres");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("postgres")), "postgres");
    }
}
