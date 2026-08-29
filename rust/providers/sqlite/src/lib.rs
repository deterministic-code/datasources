mod converters;
mod command;
mod connection;
mod datasource;
mod driver;
mod statement_builder;
#[cfg(any(test, feature = "rusqlite"))]
mod rusqlite_driver;
mod transaction;

pub use command::SqliteCommand;
pub use connection::SqliteConnection;
pub use datasource::SqliteDatasource;
pub use driver::{require_driver, SqliteDriver, SqliteExecuteResult};
pub use statement_builder::{SqliteColumn, SqliteStatementBuilder};
#[cfg(any(test, feature = "rusqlite"))]
pub use rusqlite_driver::RusqliteDriver;
pub use transaction::SqliteTransaction;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod repository_spec;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        create, register, reset, Command, Connection, Datasource, DatasourceFactory, IsolationLevel,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        assert!(require_driver(None).is_err());
        let driver: Arc<dyn SqliteDriver> = Arc::new(RusqliteDriver::new());
        require_driver(Some(driver.clone())).unwrap();
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into()), driver.clone()));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn.clone());
        command.execute(None, None).await.unwrap();
        command.execute_reader(Some("SELECT 1"), None).await.unwrap();
        command.execute_reader(None, None).await.unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        assert_eq!(
            datasources::Transaction::isolation(&*txn),
            IsolationLevel::Serializable
        );
        txn.commit().await.unwrap();
        let txn = conn
            .start_transaction(Some(IsolationLevel::ReadUncommitted))
            .await
            .unwrap();
        assert_eq!(
            datasources::Transaction::isolation(&*txn),
            IsolationLevel::ReadUncommitted
        );
        txn.abort().await.unwrap();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        assert!(conn.start_transaction(None).await.is_err());

        let shared = Arc::new(SqliteConnection::new(
            Some("file:iso?mode=memory&cache=shared".into()),
            Arc::new(RusqliteDriver::new()),
        ));
        shared.open().await.unwrap();
        let read_committed = shared
            .start_transaction(Some(IsolationLevel::ReadCommitted))
            .await
            .unwrap();
        assert_eq!(
            datasources::Transaction::isolation(&*read_committed),
            IsolationLevel::ReadCommitted
        );
        read_committed.abort().await.unwrap();
        shared.close().await.unwrap();

        let defaulted = SqliteConnection::new(None, driver.clone());
        assert_eq!(defaulted.filename, ":memory:");
        reset();
        assert_eq!(
            Datasource::provider(&*SqliteDatasource::factory()),
            "sqlite"
        );
        register(Arc::new(SqliteDatasource::new(
            Some(":memory:".into()),
            Some(driver.clone()),
            None,
        )));
        let ds = create("sqlite", None).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute(Some("DELETE FROM items"), None).await.is_err());
        assert!(command.execute_reader(Some("SELECT 1"), None).await.is_err());
        ds.create_repository(None);
        let missing = SqliteDatasource::new(None, None, None);
        assert_eq!(Datasource::provider(&missing), "sqlite");
        assert!(missing.create_connection().await.is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            missing.create_command(None)
        }))
        .is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            missing.create_repository(None)
        }))
        .is_err());
        let factory = SqliteDatasource::new(None, Some(driver.clone()), None);
        factory.create_connection().await.unwrap();
        let bound = factory.create_command(Some(factory.create_connection().await.unwrap()));
        assert!(bound.execute(Some("DELETE FROM items"), None).await.is_err());
        assert!(bound.execute_reader(Some("SELECT 1"), None).await.is_err());
        factory.create_repository(Some(factory.create_connection().await.unwrap()));
        DatasourceFactory::create(&factory, None);
        assert_eq!(DatasourceFactory::provider(&factory), "sqlite");

        let closed = Arc::new(SqliteConnection::new(None, driver));
        assert!(SqliteCommand::new(closed)
            .execute(Some("x"), None)
            .await
            .is_err());
    }
}

pub use converters::{register as register_converters, types as converter_types};
