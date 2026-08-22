use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn dynamodb_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("dynamodb")
}

pub fn register_dynamodb() {
    register(create_stub_datasource("dynamodb"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_dynamodb();
        let ds = create("dynamodb", None).unwrap();
        assert_eq!(ds.provider(), "dynamodb");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(dynamodb_datasource().provider(), "dynamodb");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("dynamodb")), "dynamodb");
    }
}
