mod command;
mod connection;
mod connection_string;
mod converters;
mod create_table_statement_renderer;
mod driver;
mod reader;
mod select_inserted_id_statement_text_renderer;
mod sqlite_path;
mod statement_renderer;
mod transaction;

pub use driver::SqliteDriver;
pub use statement_renderer::SqliteStatementRenderer;

#[cfg(test)]
pub(crate) fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = LOCK.lock().unwrap_or_else(|error| error.into_inner());
    std::env::remove_var("INSTALLED_PROVIDERS");
    std::env::remove_var("DATASOURCE_URL");
    std::env::set_var("PROVIDER", "sqlite");
    datasources::reset();
    datasources::register("sqlite", std::sync::Arc::new(crate::SqliteDriver));
    guard
}

#[cfg(test)]
#[path = "../tests/command.rs"]
mod command_tests;
#[cfg(test)]
#[path = "../tests/contacts.rs"]
mod contacts;
#[cfg(test)]
#[path = "../tests/create_from_driver.rs"]
mod create_from_driver;
#[cfg(test)]
#[path = "../tests/datasource_plural.rs"]
mod datasource_plural;
#[cfg(test)]
#[path = "../tests/integration.rs"]
mod integration;
#[cfg(test)]
#[path = "../tests/null_write.rs"]
mod null_write;
#[cfg(test)]
#[path = "../tests/read_pipeline.rs"]
mod read_pipeline;
#[cfg(test)]
#[path = "../tests/readonly_repository.rs"]
mod readonly_repository;
#[cfg(test)]
#[path = "../tests/repository_spec.rs"]
mod repository_spec;

#[cfg(test)]
mod tests {
    use crate::command::SqliteCommand;
    use crate::connection::SqliteConnection;
    use crate::transaction::SqliteTransaction;
    use datasources::{
        converters::Cells, create, downcast_connection, register, reset, BuiltDatasource, Command,
        Connection, Datasource, Driver, IsolationLevel, Transaction,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    struct With(Option<String>);

    impl Driver for With {
        fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
            Arc::new(SqliteConnection::new(self.0.clone()))
        }

        fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
            connection.create_command()
        }

        fn transaction(
            &self,
            connection: Arc<dyn Connection>,
            isolation: IsolationLevel,
        ) -> Arc<dyn Transaction> {
            let conn = downcast_connection::<SqliteConnection>(connection).expect("sqlite");
            Arc::new(SqliteTransaction::new(conn.share(), isolation))
        }

        fn row_type_converters(
            &self,
        ) -> HashMap<
            String,
            Box<
                dyn datasources::TypeConverter<datasources::Value, datasources::Value>
                    + Send
                    + Sync,
            >,
        > {
            crate::converters::row_type_converters()
        }
    }

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn.clone());
        command
            .execute_native(&datasources::Native::new(""), None)
            .await
            .unwrap();
        command
            .execute_native(
                &datasources::Native::new("CREATE TABLE t (id INTEGER)"),
                None,
            )
            .await
            .unwrap();
        command
            .execute_native(
                &datasources::Native::new("INSERT INTO t (id) VALUES (1) RETURNING id"),
                None,
            )
            .await
            .unwrap();
        command
            .execute_reader_native(&datasources::Native::new("SELECT 1"), None)
            .await
            .unwrap();
        let sets = command
            .execute_reader_native(
                &datasources::Native::new("SELECT 1 AS id; SELECT 2 AS id"),
                None,
            )
            .await
            .unwrap();
        let mut first = Vec::new();
        while sets.next().await.unwrap() {
            first.push(sets.current());
        }
        let moved = sets.next_result().await.unwrap();
        let mut second = Vec::new();
        while sets.next().await.unwrap() {
            second.push(sets.current());
        }
        assert!(moved);
        assert_eq!(
            first[0].as_ref().unwrap().get("id"),
            Some(&datasources::Value::Int(1))
        );
        assert_eq!(
            second[0].as_ref().unwrap().get("id"),
            Some(&datasources::Value::Int(2))
        );
        assert!(!sets.next_result().await.unwrap());
        drop(sets);
        command
            .execute_reader_native(&datasources::Native::new(""), None)
            .await
            .unwrap();
        let txn = conn.start_transaction(None).await.unwrap();
        assert_eq!(
            datasources::Transaction::isolation(&*txn),
            IsolationLevel::ReadCommitted
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

        let shared = Arc::new(SqliteConnection::new(Some(
            "file:iso?mode=memory&cache=shared".into(),
        )));
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

        let defaulted = SqliteConnection::new(None);
        assert_eq!(defaulted.filename, ":memory:");
        reset();
        register("sqlite", Arc::new(With(Some(":memory:".into()))));
        let ds = create(Some(&HashMap::from([("PROVIDER".into(), "sqlite".into())]))).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None).await.unwrap();
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        command
            .execute_reader_native(&datasources::Native::new("SELECT 1"), None)
            .await
            .unwrap();
        ds.create_repository::<Cells>(None, None);
        let factory = BuiltDatasource::new("sqlite", Arc::new(With(None)), None);
        factory.create_connection().await.unwrap();
        let bound = factory
            .create_command(Some(factory.create_connection().await.unwrap()))
            .await
            .unwrap();
        assert!(bound.execute("DELETE FROM items", None).await.is_err());
        assert!(bound
            .execute_reader_native(&datasources::Native::new("SELECT 1"), None)
            .await
            .is_err());
        (&factory as &dyn Datasource)
            .create_repository::<Cells>(Some(factory.create_connection().await.unwrap()), None);

        let closed = Arc::new(SqliteConnection::new(None));
        assert!(SqliteCommand::new(closed)
            .execute_native(&datasources::Native::new("x"), None)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn return_statement_selects_scalar() {
        use datasources::{Parameter, ReturnStatement};
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn);
        let parameters = [Parameter::new("x", -3_i64)];
        let response = command
            .execute_statement(&ReturnStatement::new("abs", &parameters), Some(&parameters))
            .await
            .unwrap();
        assert_eq!(response.result, datasources::Value::Int(3));
    }
}
