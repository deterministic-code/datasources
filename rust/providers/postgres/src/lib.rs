mod command;
mod connection;
mod datasource;

pub use command::PostgresCommand;
pub use connection::PostgresConnection;
pub use datasource::PostgresDatasource;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, Datasource};

    #[test]
    fn creates() {
        reset();
        register(PostgresDatasource::factory());
        assert_eq!(create("postgres", None).unwrap().provider(), "postgres");
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = PostgresDatasource.create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = PostgresDatasource;
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection));
        assert!(command.execute(None, None).await.is_err());
        assert!(command.execute_reader(None, None).await.is_err());
        let implicit = ds.create_command(None);
        assert!(implicit.execute(None, None).await.is_err());
        assert!(implicit.execute_reader(None, None).await.is_err());
    }
}
