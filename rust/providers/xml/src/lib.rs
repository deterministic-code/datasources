mod command;
mod connection;
mod export;

pub use connection::XmlDriver;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, BuiltDatasource, Datasource};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn ds() -> BuiltDatasource {
        BuiltDatasource::new("xml", Arc::new(XmlDriver), None)
    }

    #[test]
    fn creates() {
        reset();
        register("xml", Arc::new(XmlDriver));
        assert_eq!(
            create(Some(&HashMap::from([("PROVIDER".into(), "xml".into())])))
                .unwrap()
                .provider(),
            "xml"
        );
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = ds().create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = ds();
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection)).await.unwrap();
        assert!(command.execute("", None).await.is_err());
        assert!(command.execute_reader("", None).await.is_err());
        assert!(ds.create_command(None).await.is_err());
    }
}
