mod alter_table_statement_renderer;
mod command;
mod connection;
mod converters;
mod create_table_statement_renderer;
mod create_trigger_statement_renderer;
mod drop_routine_statement_renderer;
mod driver;
mod default_value_converter;
mod default_value_options;
mod sql_generate;
mod type_map;
mod isolation_sql;
mod reader;
mod select_inserted_id_statement_text_renderer;
mod sql_seed_statement_renderer;
mod statement_renderer;
mod stored_procedure_statement_renderer;
mod transaction;

pub use driver::MysqlDriver;

#[cfg(test)]
#[path = "../tests/isolate.rs"]
mod isolate;
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
#[path = "../tests/generate_sql.rs"]
mod generate_sql_tests;

#[cfg(test)]
mod tests {
    use crate::command::MysqlCommand;
    use crate::connection::MysqlConnection;
    use crate::isolation_sql::IsolationSql;
    use crate::MysqlDriver;
    use crate::transaction::MysqlTransaction;
    use datasources::{
        InsertStatement, Parameter, Row, create, downcast_connection, register, reset,
        BuiltDatasource, Command, Connection, Datasource, Driver, IsolationLevel, Transaction,
        Value,
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
        let statements = MysqlDriver::statement_text_renderer();
        assert_eq!(
            statements.render(&InsertStatement::new("users", &[Parameter::new("id", Value::Null)])),
            "INSERT INTO users (id) VALUES (:id)"
        );
        assert_eq!(
            statements.render(&InsertStatement::new(
                "groups",
                &[
                    Parameter::new("description", Value::Null),
                    Parameter::new("name", Value::Null),
                ]
            )),
            "INSERT INTO `groups` (description, name) VALUES (:description, :name)"
        );
        assert_eq!(
            statements.render(&InsertStatement::new(
                "contact_settings",
                &[
                    Parameter::new("contact_id", Value::Null),
                    Parameter::new("key", Value::Null),
                    Parameter::new("value", Value::Null),
                ]
            )),
            "INSERT INTO contact_settings (contact_id, `key`, value) VALUES (:contact_id, :key, :value)"
        );
        assert_eq!(
            statements.render(&InsertStatement::new(
                "OldContactsTbl",
                &[Parameter::new("CntID", Value::Null)]
            )),
            "INSERT INTO `OldContactsTbl` (`CntID`) VALUES (:CntID)"
        );
        assert_eq!(IsolationSql::of(IsolationLevel::ReadUncommitted), "READ UNCOMMITTED");
        assert_eq!(IsolationSql::of(IsolationLevel::ReadCommitted), "READ COMMITTED");
        assert_eq!(IsolationSql::of(IsolationLevel::RepeatableRead), "REPEATABLE READ");
        assert_eq!(IsolationSql::of(IsolationLevel::Serializable), "REPEATABLE READ");
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
        ds.create_repository::<Row>(None, None);
        let factory = BuiltDatasource::new("mysql", Arc::new(With(None)), None);
        assert_eq!(factory.provider(), "mysql");
        let closed = Arc::new(MysqlConnection::new(None));
        assert!(MysqlCommand::new(closed).execute_native(&datasources::Native::new("x"), None).await.is_err());
    }
}
