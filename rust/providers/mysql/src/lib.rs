mod command;
mod command_driver;
mod connection;
mod connection_driver;
mod converters;
mod driver;
mod provider;
mod mysql_async_driver;
mod transaction;
mod transaction_driver;

pub use provider::MysqlProvider;

#[cfg(test)]
#[path = "../tests/integration.rs"]
mod integration;
#[cfg(test)]
#[path = "../tests/repository_spec.rs"]
mod repository_spec;

#[cfg(test)]
mod tests {
    use crate::command::MysqlCommand;
    use crate::connection::MysqlConnection;
    use crate::driver::{isolation_sql, require_driver, to_mysql, MysqlDriver, MysqlExecuteResult};
    use crate::transaction::MysqlTransaction;
    use async_trait::async_trait;
    use datasources::{
        create, register, reset, BuiltDatasource, Command, Connection, Datasource, Driver,
        IsolationLevel, Transaction, Result, Row, Value,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    struct With(Arc<dyn MysqlDriver>, Option<String>);

    impl Driver for With {
        fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
            Arc::new(MysqlConnection::new(self.1.clone(), self.0.clone()))
        }

        fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
            connection.create_command()
        }

        fn transaction(
            &self,
            connection: Arc<dyn Connection>,
            isolation: IsolationLevel,
        ) -> Arc<dyn Transaction> {
            let conn = datasources::downcast_connection::<MysqlConnection>(connection).expect("mysql");
            Arc::new(MysqlTransaction::new(conn.driver.clone(), isolation))
        }

        fn row_type_converters(
            &self,
        ) -> HashMap<String, Box<dyn datasources::TypeConverter<datasources::Value, datasources::Value> + Send + Sync>>
        {
            crate::converters::row_type_converters()
        }
    }

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
        assert_eq!(
            isolation_sql(IsolationLevel::ReadCommitted),
            "READ COMMITTED"
        );
        assert_eq!(
            isolation_sql(IsolationLevel::RepeatableRead),
            "REPEATABLE READ"
        );
        assert_eq!(
            isolation_sql(IsolationLevel::Serializable),
            "REPEATABLE READ"
        );
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
        let driver_for_ds = driver.clone();
        register(
            "mysql",
            Arc::new(With(driver_for_ds, Some("mysql://db/app".into()))),
        );
        let ds = create(Some(&HashMap::from([("provider".into(), "mysql".into())]))).unwrap();
        assert_eq!(ds.provider(), "mysql");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        assert!(command.execute_reader("SELECT 1", None).await.is_err());
        ds.create_repository(None);
        let factory = BuiltDatasource::new("mysql", Arc::new(With(driver.clone(), None)), None);
        factory.create_connection().await.unwrap();
        let closed = Arc::new(MysqlConnection::new(None, driver));
        assert!(MysqlCommand::new(closed).execute("x", None).await.is_err());
    }
}

