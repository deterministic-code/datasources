mod bind;
mod command;
mod connection;
mod converters;
mod create_table_statement_renderer;
mod driver;
mod pg_value;
mod reader;
mod select_inserted_id_statement_text_renderer;
mod statement_renderer;
mod transaction;

pub use driver::PostgresDriver;

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
    use crate::bind::{isolation_sql, to_postgres};
    use crate::command::PostgresCommand;
    use crate::connection::PostgresConnection;
    use crate::transaction::PostgresTransaction;
    use datasources::{
        converters::Cells, create, downcast_connection, register, reset, Command, Connection,
        Driver, IsolationLevel, Transaction, Value,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    struct With(Option<String>);

    impl Driver for With {
        fn connection(&self, _: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
            Arc::new(PostgresConnection::new(self.0.clone(), None))
        }

        fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
            connection.create_command()
        }

        fn transaction(
            &self,
            connection: Arc<dyn Connection>,
            isolation: IsolationLevel,
        ) -> Arc<dyn Transaction> {
            let conn = downcast_connection::<PostgresConnection>(connection).expect("postgres");
            Arc::new(PostgresTransaction::new(conn.share(), isolation))
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
    async fn postgres_command_and_repository() {
        let (sql, values) = to_postgres(
            "SELECT * FROM t WHERE id = $id AND n = $id",
            &HashMap::from([("id".into(), Value::Int(1))]),
        );
        assert_eq!(sql, "SELECT * FROM t WHERE id = $1 AND n = $1");
        assert_eq!(values, vec![Value::Int(1)]);
        let (_, missing) = to_postgres("SELECT $missing", &HashMap::new());
        assert_eq!(missing, vec![Value::Null]);
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
        assert_eq!(isolation_sql(IsolationLevel::Serializable), "SERIALIZABLE");
        let conn = Arc::new(PostgresConnection::new(
            Some("postgres://db/app".into()),
            Some("s".into()),
        ));
        assert_eq!(conn.schema.as_deref(), Some("s"));
        let command = PostgresCommand::new(conn.clone());
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
        let defaulted = PostgresConnection::new(None, None);
        assert_eq!(defaulted.url, "");
        let options = HashMap::from([(
            "DATASOURCE_URL".into(),
            "postgres://db/app?schema=s".into(),
        )]);
        let from = downcast_connection::<PostgresConnection>(PostgresConnection::from_options(
            Some(&options),
        ))
        .expect("postgres");
        assert_eq!(from.url, "postgres://db/app?schema=s");
        assert_eq!(from.schema.as_deref(), Some("s"));
        reset();
        register("postgres", Arc::new(With(Some("postgres://db/app".into()))));
        let ds = create(Some(&HashMap::from([(
            "PROVIDER".into(),
            "postgres".into(),
        )])))
        .unwrap();
        assert_eq!(ds.provider(), "postgres");
        assert!(ds.create_command(None).await.is_err());
        ds.create_repository::<Cells>(None, None);
        let closed = Arc::new(PostgresConnection::new(None, None));
        assert!(PostgresCommand::new(closed)
            .execute_native(&datasources::Native::new("x"), None)
            .await
            .is_err());
    }
}
