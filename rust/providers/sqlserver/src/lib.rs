use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn sqlserver_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("sqlserver")
}

pub fn register_sqlserver() {
    register(create_stub_datasource("sqlserver"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_sqlserver();
        let ds = create("sqlserver", None).unwrap();
        assert_eq!(ds.provider(), "sqlserver");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(sqlserver_datasource().provider(), "sqlserver");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("sqlserver")), "sqlserver");
    }
}
