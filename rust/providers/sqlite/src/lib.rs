mod command;
mod connection;
mod datasource;
mod driver;
mod repository;
mod transaction;

pub use command::SqliteCommand;
pub use connection::SqliteConnection;
pub use datasource::SqliteDatasource;
pub use driver::{require_driver, ScriptedDriver, SqliteDriver, SqliteExecuteResult};
pub use repository::SqliteRepository;
pub use transaction::SqliteTransaction;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        create, register, reset, Command, Connection, Datasource, DatasourceFactory, MiddlewareAware,
        Parameter, Record, Repository, TextQuery, TextWhere, Value,
    };
    use std::sync::Arc;

    fn rec(id: i64) -> Record {
        let mut r = Record::new();
        r.insert("id".into(), Value::Int(id));
        r
    }

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        assert!(require_driver(None).is_err());
        let driver = Arc::new(ScriptedDriver::new());
        require_driver(Some(driver.clone())).unwrap();
        driver
            .query_result
            .lock()
            .unwrap()
            .push(rec(1));
        let conn = Arc::new(SqliteConnection::new(Some("x.db".into()), driver.clone()));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn.clone());
        command
            .execute(Some("DELETE FROM items"), Some(&[Parameter::new("id", 1_i64)]))
            .await
            .unwrap();
        command.execute(None, None).await.unwrap();
        command.execute_reader(Some("SELECT 1"), None).await.unwrap();
        command.execute_reader(None, None).await.unwrap();
        command.use_middleware(None, None);

        let repo = SqliteRepository::new(conn.clone(), None);
        assert!(repo.get_single(None).await.unwrap().is_some());
        driver.query_result.lock().unwrap().clear();
        assert!(repo
            .get_single(Some(&TextQuery::new("id = 1")))
            .await
            .unwrap()
            .is_none());
        driver.query_result.lock().unwrap().push(rec(1));
        assert_eq!(repo.get(None).await.unwrap().len(), 1);
        repo.add(&rec(2)).await.unwrap();
        repo.add(&Record::new()).await.unwrap();
        repo.update(&rec(2), None).await.unwrap();
        repo.update(&Record::new(), Some(&TextWhere::new("id = 1")))
            .await
            .unwrap();
        repo.delete(&rec(2), None).await.unwrap();
        repo.delete(&rec(2), Some(&TextWhere::new("name = $name")))
            .await
            .unwrap();
        let named = SqliteRepository::new(conn.clone(), Some("users".into()));
        named.get(None).await.unwrap();
        assert!(named.table == "users");

        let txn = conn.start_transaction().await.unwrap();
        txn.commit().await.unwrap();
        let txn = conn.start_transaction().await.unwrap();
        txn.abort().await.unwrap();
        txn.use_middleware(None, None);
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        assert!(conn.start_transaction().await.is_err());

        let defaulted = SqliteConnection::new(None, driver.clone());
        assert_eq!(defaulted.filename, ":memory:");
        defaulted.use_middleware(None, None);

        reset();
        register(SqliteDatasource::factory());
        let ds = create("sqlite", None).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute(Some("DELETE FROM items"), None).await.is_err());
        assert!(command.execute_reader(Some("SELECT 1"), None).await.is_err());
        ds.create_repository(None);
        let factory = SqliteDatasource::new(None, None, None);
        assert_eq!(Datasource::provider(&factory), "sqlite");
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
