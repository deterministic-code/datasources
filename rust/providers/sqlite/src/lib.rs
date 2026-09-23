mod command;
mod connection;
mod connection_string;
mod converters;
mod driver;
mod sqlite_path;
mod transaction;

pub use driver::SqliteDriver;

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
    use crate::command::SqliteCommand;
    use crate::connection::SqliteConnection;
    use crate::transaction::SqliteTransaction;
    use datasources::{
        create, downcast_connection, register, reset, BuiltDatasource, Command, Connection,
        Datasource, Driver, IsolationLevel, Transaction,
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
        ) -> HashMap<String, Box<dyn datasources::TypeConverter<datasources::Value, datasources::Value> + Send + Sync>>
        {
            crate::converters::row_type_converters()
        }
    }

    #[tokio::test]
    async fn sqlite_command_and_repository() {
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
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
        let ds = create(Some(&HashMap::from([("provider".into(), "sqlite".into())]))).unwrap();
        assert_eq!(ds.provider(), "sqlite");
        ds.create_connection().await.unwrap();
        let command = ds.create_command(None);
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        assert!(command.execute_reader("SELECT 1", None).await.is_err());
        ds.create_repository(None);
        let factory = BuiltDatasource::new("sqlite", Arc::new(With(None)), None);
        factory.create_connection().await.unwrap();
        let bound = factory.create_command(Some(factory.create_connection().await.unwrap()));
        assert!(bound.execute("DELETE FROM items", None).await.is_err());
        assert!(bound.execute_reader("SELECT 1", None).await.is_err());
        factory.create_repository(Some(factory.create_connection().await.unwrap()));

        let closed = Arc::new(SqliteConnection::new(None));
        assert!(SqliteCommand::new(closed).execute("x", None).await.is_err());
    }

    #[tokio::test]
    async fn return_statement_selects_scalar() {
        use datasources::{Parameter, ReturnStatement, Text};
        let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
        conn.open().await.unwrap();
        let command = SqliteCommand::new(conn);
        let parameters = [Parameter::new("x", -3_i64)];
        let response = command
            .execute(&ReturnStatement::new("abs", &parameters).get_text(), Some(&parameters))
            .await
            .unwrap();
        assert_eq!(response.result, datasources::Value::Int(3));
    }
}

