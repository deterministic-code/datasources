mod command;
mod connection;
mod converters;
mod driver;
mod engine;
mod transaction;

pub use driver::MysqlDriver;

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
    use crate::engine::{isolation_sql, to_mysql};
    use crate::transaction::MysqlTransaction;
    use datasources::{
        converters::Cells, create, downcast_connection, register, reset, BuiltDatasource, Command,
        Connection, Datasource, Driver, IsolationLevel, Transaction,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    struct With(Option<String>);

    impl Driver for With {
        fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
            Arc::new(MysqlConnection::new(self.0.clone()))
        }

        fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
            connection.create_command()
        }

        fn transaction(
            &self,
            connection: Arc<dyn Connection>,
            isolation: IsolationLevel,
        ) -> Arc<dyn Transaction> {
            let conn = downcast_connection::<MysqlConnection>(connection).expect("mysql");
            Arc::new(MysqlTransaction::new(conn.share(), isolation))
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
    async fn mysql_command_and_repository() {
        assert_eq!(
            to_mysql("SELECT * FROM t WHERE id = $id"),
            "SELECT * FROM t WHERE id = :id"
        );
        assert_eq!(
            to_mysql("INSERT INTO t (id) VALUES ($id) RETURNING id"),
            "INSERT INTO t (id) VALUES (:id)"
        );
        assert_eq!(to_mysql("SELECT $1"), "SELECT $1");
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
        let conn = Arc::new(MysqlConnection::new(Some("mysql://db/app".into())));
        let command = MysqlCommand::new(conn.clone());
        command.execute("", None).await.unwrap();
        command.execute_reader("", None).await.unwrap();
        assert!(conn.start_transaction(None).await.is_err());
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        let defaulted = MysqlConnection::new(None);
        assert_eq!(defaulted.url, "");
        let options = HashMap::from([("database_url".into(), "mysql://db/app".into())]);
        let from =
            downcast_connection::<MysqlConnection>(MysqlConnection::from_options(Some(&options)))
                .expect("mysql");
        assert_eq!(from.url, "mysql://db/app");
        reset();
        register("mysql", Arc::new(With(Some("mysql://db/app".into()))));
        let ds = create(Some(&HashMap::from([("provider".into(), "mysql".into())]))).unwrap();
        assert_eq!(ds.provider(), "mysql");
        let command = ds.create_command(None);
        assert!(command.execute("DELETE FROM items", None).await.is_err());
        assert!(command.execute_reader("SELECT 1", None).await.is_err());
        ds.create_repository::<Cells>(None);
        let factory = BuiltDatasource::new("mysql", Arc::new(With(None)), None);
        assert_eq!(factory.provider(), "mysql");
        let closed = Arc::new(MysqlConnection::new(None));
        assert!(MysqlCommand::new(closed).execute("x", None).await.is_err());
    }
}
