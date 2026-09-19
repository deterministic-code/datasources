mod command;
mod command_driver;
mod connection;
mod connection_driver;
mod connection_string;
mod converters;
mod driver;
mod rusqlite_driver;
mod sqlite_path;
mod statement_builder;
mod transaction;
mod transaction_driver;

pub use command::SqliteCommand;
pub use connection::SqliteConnection;
pub use driver::{require_driver, SqliteDriver, SqliteExecuteResult};
pub use rusqlite_driver::RusqliteDriver;
pub use statement_builder::{SqliteColumn, SqliteStatementBuilder};
pub use transaction::SqliteTransaction;

#[cfg(test)]
#[path = "../tests/contacts.rs"]
mod contacts;
#[cfg(test)]
#[path = "../tests/integration.rs"]
mod integration;
#[cfg(test)]
#[path = "../tests/repository_spec.rs"]
mod repository_spec;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        create, register, reset, BuiltDatasource, Command, Connection, Datasource, IsolationLevel,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        assert!(require_driver(None).is_err());
        let driver: Arc<dyn SqliteDriver> = Arc::new(RusqliteDriver::new());
        require_driver(Some(driver.clone())).unwrap();
        let conn = Arc::new(SqliteConnection::new(
            Some(":memory:".into()),
            driver.clone(),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn.clone());
        command.execute("", None).await.unwrap();
        command
            .execute("CREATE TABLE t (id INTEGER)", None)
            .await
            .unwrap();
        command
            .execute("INSERT INTO t (id) VALUES (1) RETURNING id", None)
            .await
            .unwrap();
        command.execute_reader("SELECT 1", None).await.unwrap();
        command.execute_reader("", None).await.unwrap();
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
        let driver_for_ds = driver.clone();
        register(
            "sqlite",
            move |_| {
                Arc::new(SqliteConnection::new(
                    Some(":memory:".into()),
                    driver_for_ds.clone(),
                ))
            },
            SqliteConnection::repository,
        );
        let ds = create(Some(&HashMap::from([("provider".into(), "sqlite".into())]))).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        assert!(command.execute_reader("SELECT 1", None).await.is_err());
        ds.create_repository(None);
        let factory = BuiltDatasource::new(
            "sqlite",
            {
                let driver = driver.clone();
                move || Arc::new(SqliteConnection::new(None, driver.clone()))
            },
            SqliteConnection::repository,
        );
        factory.create_connection().await.unwrap();
        let bound = factory.create_command(Some(factory.create_connection().await.unwrap()));
        assert!(bound.execute("DELETE FROM items", None).await.is_err());
        assert!(bound.execute_reader("SELECT 1", None).await.is_err());
        factory.create_repository(Some(factory.create_connection().await.unwrap()));

        let closed = Arc::new(SqliteConnection::new(None, driver));
        assert!(SqliteCommand::new(closed).execute("x", None).await.is_err());
    }
}

pub use converters::{register as register_converters, types as converter_types};
