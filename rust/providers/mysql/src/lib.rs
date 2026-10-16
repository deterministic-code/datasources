mod alter_table_statement_renderer;
mod command;
mod connection;
mod converters;
mod create_routine_statement_renderer;
mod create_table_statement_renderer;
mod create_trigger_statement_renderer;
mod drop_routine_statement_renderer;
mod driver;
mod sql_generate;
mod engine;
mod reader;
mod select_inserted_id_statement_text_renderer;
mod statement_renderer;
mod transaction;

pub use driver::MysqlDriver;
pub use sql_generate::{DEFAULTS, QUOTE_LEFT, QUOTE_RIGHT, TYPES};

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
            to_mysql("INSERT INTO contact_settings (contact_id, key, value) VALUES ($contact_id, $key, $value)"),
            "INSERT INTO contact_settings (contact_id, `key`, value) VALUES (:contact_id, :key, :value)"
        );
        assert_eq!(to_mysql("CREATE TABLE t (id INT PRIMARY KEY)"), "CREATE TABLE t (id INT PRIMARY KEY)");
        assert_eq!(
            to_mysql("CREATE TABLE t (id INT AUTO_INCREMENT, CONSTRAINT `pk` PRIMARY KEY (`id`))"),
            "CREATE TABLE t (id INT AUTO_INCREMENT, CONSTRAINT `pk` PRIMARY KEY (`id`))"
        );
        assert_eq!(
            to_mysql("INSERT INTO OldContactsTbl (CntID) VALUES ($CntID)"),
            "INSERT INTO `OldContactsTbl` (`CntID`) VALUES (:cntid)"
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
        let conn = Arc::new(MysqlConnection::new(Some("mysql://db/app".into())));
        let command = MysqlCommand::new(conn.clone());
        command
            .execute_native(&datasources::Native::new(""), None)
            .await
            .unwrap();
        command
            .execute_reader_native(&datasources::Native::new(""), None)
            .await
            .unwrap();
        assert!(conn.start_transaction(None).await.is_err());
        conn.close().await.unwrap();
        assert!(conn.ensure_open().is_err());
        let defaulted = MysqlConnection::new(None);
        assert_eq!(defaulted.url, "");
        let options = HashMap::from([("DATASOURCE_URL".into(), "mysql://db/app".into())]);
        let from =
            downcast_connection::<MysqlConnection>(MysqlConnection::from_options(Some(&options)))
                .expect("mysql");
        assert_eq!(from.url, "mysql://db/app");
        reset();
        register("mysql", Arc::new(With(Some("mysql://db/app".into()))));
        let ds = create(Some(&HashMap::from([("PROVIDER".into(), "mysql".into())]))).unwrap();
        assert_eq!(ds.provider(), "mysql");
        assert!(ds.create_command(None).await.is_err());
        ds.create_repository::<Cells>(None, None);
        let factory = BuiltDatasource::new("mysql", Arc::new(With(None)), None);
        assert_eq!(factory.provider(), "mysql");
        let closed = Arc::new(MysqlConnection::new(None));
        assert!(MysqlCommand::new(closed).execute_native(&datasources::Native::new("x"), None).await.is_err());
    }
}
