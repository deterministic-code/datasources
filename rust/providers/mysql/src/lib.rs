mod command;
mod connection;
mod datasource;

pub use command::MysqlCommand;
pub use connection::MysqlConnection;
pub use datasource::MysqlDatasource;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, Datasource};

    #[test]
    fn creates() {
        reset();
        register(MysqlDatasource::factory());
        assert_eq!(create("mysql", None).unwrap().provider(), "mysql");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = MysqlDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = MysqlDatasource;
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection));
        assert!(command.execute(None, None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute(None, None).await.is_err());
        assert!(implicit.execute_reader(None, None).await.is_err());
    }
}
