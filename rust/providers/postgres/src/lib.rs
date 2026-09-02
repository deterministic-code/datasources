mod converters;
mod command;
mod connection;
mod datasource;
mod driver;
mod repository;
mod statement_builder;
mod tokio_postgres_driver;
mod transaction;

pub use command::PostgresCommand;
pub use connection::PostgresConnection;
pub use datasource::PostgresDatasource;
pub use driver::{
    isolation_sql, require_driver, to_postgres, with_returning, PostgresDriver, PostgresExecuteResult,
};
pub use repository::PostgresRepository;
pub use statement_builder::{PostgresColumn, PostgresStatementBuilder};
pub use tokio_postgres_driver::TokioPostgresDriver;
pub use transaction::PostgresTransaction;

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
    impl PostgresDriver for FakeDriver {
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
        ) -> Result<PostgresExecuteResult> {
            Ok(if sql.trim().is_empty() {
                PostgresExecuteResult {
                    rows_affected: 0,
                    result: Value::Null,
                }
            } else {
                PostgresExecuteResult {
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
    async fn postgres_command_and_repository() {
        assert!(require_driver(None).is_err());
        let (sql, values) = to_postgres(
            "SELECT * FROM t WHERE id = $id AND n = $id",
            &HashMap::from([("id".into(), Value::Int(1))]),
        );
        assert_eq!(sql, "SELECT * FROM t WHERE id = $1 AND n = $1");
        assert_eq!(values, vec![Value::Int(1)]);
        assert_eq!(
            with_returning("INSERT INTO t (id) VALUES (1)"),
            "INSERT INTO t (id) VALUES (1) RETURNING id"
        );
        assert_eq!(
            with_returning("INSERT INTO t VALUES (1) RETURNING id"),
            "INSERT INTO t VALUES (1) RETURNING id"
        );
        assert_eq!(with_returning("UPDATE t SET x = 1"), "UPDATE t SET x = 1");
        assert_eq!(
            isolation_sql(IsolationLevel::ReadUncommitted),
            "READ UNCOMMITTED"
        );
        assert_eq!(isolation_sql(IsolationLevel::ReadCommitted), "READ COMMITTED");
        assert_eq!(
            isolation_sql(IsolationLevel::RepeatableRead),
            "REPEATABLE READ"
        );
        assert_eq!(isolation_sql(IsolationLevel::Serializable), "SERIALIZABLE");
        let driver: Arc<dyn PostgresDriver> = Arc::new(FakeDriver);
        let conn = Arc::new(PostgresConnection::new(
            Some("postgres://db/app".into()),
            Some("s".into()),
            driver.clone(),
        ));
        conn.open().await.unwrap();
        conn.open().await.unwrap();
        assert_eq!(conn.schema.as_deref(), Some("s"));
        let command = PostgresCommand::new(conn.clone());
        command.execute("", None).await.unwrap();
        command.execute_reader("SELECT 1", None).await.unwrap();
        command.execute_reader("", None).await.unwrap();
        assert_eq!(PostgresRepository::new(conn.clone(), None).table, "items");
        let txn = conn.start_transaction(None).await.unwrap();
        txn.commit().await.unwrap();
        conn.start_transaction(Some(IsolationLevel::ReadUncommitted))
            .await
            .unwrap()
            .abort()
            .await
            .unwrap();
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        let defaulted = PostgresConnection::new(None, None, driver.clone());
        assert_eq!(defaulted.url, "");
        reset();
        register(Arc::new(PostgresDatasource::new(
            Some("postgres://db/app".into()),
            None,
            Some(driver.clone()),
            None,
        )));
        let ds = create("postgres", None).unwrap();
        assert_eq!(ds.provider(), "postgres");
        assert!(ds
            .create_command(None)
            .execute("DELETE FROM items", None)
            .await
            .is_err());
        ds.create_repository(None);
        let missing = PostgresDatasource::new(None, None, None, None);
        assert_eq!(Datasource::provider(&missing), "postgres");
        assert!(missing.create_connection().await.is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            missing.create_command(None)
        }))
        .is_err());
        let factory = PostgresDatasource::new(None, None, Some(driver.clone()), Some("people".into()));
        DatasourceFactory::create(&factory, None);
        assert_eq!(DatasourceFactory::provider(&factory), "postgres");
        let closed = Arc::new(PostgresConnection::new(None, None, driver));
        assert!(PostgresCommand::new(closed)
            .execute("x", None)
            .await
            .is_err());
    }
}

pub use converters::{register as register_converters, types as converter_types};
