mod command;
mod connection;
mod converters;

pub use command::OracleCommand;
pub use connection::{OracleConnection, OracleProvider};

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, BuiltDatasource, Datasource};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn ds() -> BuiltDatasource {
        BuiltDatasource::new("oracle", Arc::new(OracleProvider), None)
    }

    #[test]
    fn creates() {
        reset();
        register("oracle", Arc::new(OracleProvider));
        assert_eq!(
            create(Some(&HashMap::from([("provider".into(), "oracle".into())])))
                .unwrap()
                .provider(),
            "oracle"
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

pub use converters::{register as register_converters, types as converter_types};
