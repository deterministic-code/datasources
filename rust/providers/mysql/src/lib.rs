use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn mysql_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("mysql")
}

pub fn register_mysql() {
    register(create_stub_datasource("mysql"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_mysql();
        let ds = create("mysql", None).unwrap();
        assert_eq!(ds.provider(), "mysql");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(mysql_datasource().provider(), "mysql");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("mysql")), "mysql");
    }
}
