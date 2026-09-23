mod command;
mod connection;
mod converters;

pub use command::XmlCommand;
pub use connection::{XmlConnection, XmlProvider};

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, BuiltDatasource, Datasource};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn ds() -> BuiltDatasource {
        BuiltDatasource::new("xml", Arc::new(XmlProvider), None)
    }

    #[test]
    fn creates() {
        reset();
        register("xml", Arc::new(XmlProvider));
        assert_eq!(
            create(Some(&HashMap::from([("provider".into(), "xml".into())])))
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
        let command = ds.create_command(Some(connection));
        assert!(command.execute("", None).await.is_err());
        assert!(command.execute_reader("", None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute("", None).await.is_err());
        assert!(implicit.execute_reader("", None).await.is_err());
    }
}

pub use converters::{register as register_converters, row_type_converters as converter_types};
