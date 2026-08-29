mod converters;
mod command;
mod connection;
mod datasource;

pub use command::YamlCommand;
pub use connection::YamlConnection;
pub use datasource::YamlDatasource;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, Datasource};

    #[test]
    fn creates() {
        reset();
        register(YamlDatasource::instance());
        assert_eq!(create("yaml", None).unwrap().provider(), "yaml");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = YamlDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = YamlDatasource;
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection));
        assert!(command.execute(None, None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute(None, None).await.is_err());
        assert!(implicit.execute_reader(None, None).await.is_err());
    }
}

pub use converters::{register as register_converters, types as converter_types};
