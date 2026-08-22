use datasources::{create_stub_datasource, register, Datasource};
use std::sync::Arc;

pub fn text_datasource() -> Arc<dyn Datasource> {
    create_stub_datasource("text")
}

pub fn register_text() {
    register(create_stub_datasource("text"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_stub_datasource, DatasourceFactory, reset};

    #[tokio::test]
    async fn registers_and_throws() {
        reset();
        register_text();
        let ds = create("text", None).unwrap();
        assert_eq!(ds.provider(), "text");
        assert!(ds.create_connection().open().await.is_err());
        assert_eq!(text_datasource().provider(), "text");
        assert_eq!(DatasourceFactory::provider(&*create_stub_datasource("text")), "text");
    }
}
