mod converters;
mod command;
mod connection;
mod datasource;

pub use command::OracleCommand;
pub use connection::OracleConnection;
pub use datasource::OracleDatasource;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, Datasource};

    #[test]
    fn creates() {
        reset();
        register(OracleDatasource::instance());
        assert_eq!(create("oracle", None).unwrap().provider(), "oracle");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = OracleDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = OracleDatasource;
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
