mod converters;
mod command;
mod connection;
mod datasource;
mod driver;
mod mysql_async_driver;
mod statement_builder;
mod transaction;

pub use command::MysqlCommand;
pub use connection::MysqlConnection;
pub use datasource::MysqlDatasource;
pub use driver::{isolation_sql, require_driver, to_mysql, MysqlDriver, MysqlExecuteResult};
pub use mysql_async_driver::MysqlAsyncDriver;
pub use statement_builder::{MysqlColumn, MysqlStatementBuilder};
pub use transaction::MysqlTransaction;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod repository_spec;

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use datasources::{
        create, register, reset, Command, Connection, Datasource, DatasourceFactory, IsolationLevel,
        Row, Result, Value,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    struct FakeDriver;

    #[async_trait]
    impl MysqlDriver for FakeDriver {
        async fn open(&self, _url: &str) -> Result<()> {
            Ok(())
        }
        async fn close(&self) -> Result<()> {
            Ok(())
        }
        async fn execute(
            &self,
            sql: &str,
            _params: &HashMap<String, Value>,
        ) -> Result<MysqlExecuteResult> {
            Ok(if sql.trim().is_empty() {
                MysqlExecuteResult {
                    rows_affected: 0,
                    result: Value::Null,
                }
            } else {
                MysqlExecuteResult {
                    rows_affected: 2,
                    result: Value::Int(5),
                }
            })
        }
        async fn query(&self, sql: &str, _params: &HashMap<String, Value>) -> Result<Vec<Row>> {
            Ok(if sql.trim().is_empty() {
                vec![]
            } else {
                let mut row = Row::new();
                row.insert("id".into(), Value::Int(1));
                vec![row]
            })
        }
        async fn begin(&self, _isolation: IsolationLevel) -> Result<()> {
            Ok(())
        }
        async fn commit(&self) -> Result<()> {
            Ok(())
        }
        async fn rollback(&self) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn mysql_command_and_repository() {
        assert!(require_driver(None).is_err());
        assert_eq!(
            to_mysql("SELECT * FROM t WHERE id = $id"),
            "SELECT * FROM t WHERE id = :id"
        );
        assert_eq!(
            to_mysql("INSERT INTO t (id) VALUES ($id) RETURNING id"),
            "INSERT INTO t (id) VALUES (:id)"
        );
        assert_eq!(
            isolation_sql(IsolationLevel::ReadUncommitted),
            "READ UNCOMMITTED"
        );
        assert_eq!(isolation_sql(IsolationLevel::ReadCommitted), "READ COMMITTED");
        assert_eq!(
            isolation_sql(IsolationLevel::RepeatableRead),
            "REPEATABLE READ"
        );
        assert_eq!(isolation_sql(IsolationLevel::Serializable), "REPEATABLE READ");
        let driver: Arc<dyn MysqlDriver> = Arc::new(FakeDriver);
        require_driver(Some(driver.clone())).unwrap();
        let conn = Arc::new(MysqlConnection::new(
            Some("mysql://db/app".into()),
            driver.clone(),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        let command = MysqlCommand::new(conn.clone());
        command.execute("", None).await.unwrap();
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
        txn.abort().await.unwrap();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        assert!(conn.start_transaction(None).await.is_err());
        let defaulted = MysqlConnection::new(None, driver.clone());
        assert_eq!(defaulted.url, "");
        reset();
        register(Arc::new(MysqlDatasource::new(
            Some("mysql://db/app".into()),
            Some(driver.clone()),
            None,
        )));
        let ds = create("mysql", None).unwrap();
        assert_eq!(ds.provider(), "mysql");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        assert!(command.execute_reader("SELECT 1", None).await.is_err());
        ds.create_repository(None);
        let missing = MysqlDatasource::new(None, None, None);
        assert_eq!(Datasource::provider(&missing), "mysql");
        assert!(missing.create_connection().await.is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            missing.create_command(None)
        }))
        .is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            missing.create_repository(None)
        }))
        .is_err());
        let factory = MysqlDatasource::new(None, Some(driver.clone()), Some("people".into()));
        factory.create_connection().await.unwrap();
        DatasourceFactory::create(&factory, None);
        assert_eq!(DatasourceFactory::provider(&factory), "mysql");
        let closed = Arc::new(MysqlConnection::new(None, driver));
        assert!(MysqlCommand::new(closed)
            .execute("x", None)
            .await
            .is_err());
    }
}

pub use converters::{register as register_converters, types as converter_types};
