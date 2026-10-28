mod alter_table_statement_renderer;
mod bind;
mod command;
mod connection;
mod converters;
mod create_table_statement_renderer;
mod create_trigger_statement_renderer;
mod drop_routine_statement_renderer;
mod driver;
mod identity_insert_statement_renderer;
mod sql_generate;
mod pg_value;
mod reader;
mod select_inserted_id_statement_text_renderer;
mod statement_renderer;
mod stored_procedure_statement_renderer;
mod transaction;

pub use driver::PostgresDriver;

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
mod generate_sql;
#[cfg(test)]
#[path = "../tests/generate_sql_casing.rs"]
mod generate_sql_casing;
#[cfg(test)]
#[path = "../tests/generate_sql_mapping.rs"]
mod generate_sql_mapping;

#[cfg(test)]
mod tests {
    use crate::bind::{param_type, to_postgres};
    use crate::connection::isolation_sql;
    use crate::converters::BinaryConverter;
    use tokio_postgres::types::Type;
    use crate::command::PostgresCommand;
    use crate::connection::PostgresConnection;
    use crate::transaction::PostgresTransaction;
    use datasources::{
        converters::Cells, create, downcast_connection, register, reset, Command, Connection,
        Driver, IsolationLevel, Transaction, TypeConverter, Value,
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
        let (seed, _) = to_postgres(
            "INSERT INTO \"contact_sources\" (\"name\") VALUES ('Manual')",
            &HashMap::new(),
        );
        assert_eq!(seed, "INSERT INTO \"contact_sources\" (\"name\") VALUES ('Manual')");
        let (legacy, _) = to_postgres("SELECT * FROM OldContactsTbl", &HashMap::new());
        assert_eq!(legacy, "SELECT * FROM \"OldContactsTbl\"");
        let (_, missing) = to_postgres("SELECT $missing", &HashMap::new());
        assert_eq!(missing, vec![Value::Null]);
        let stamp = Value::from("2020-01-02T03:04:05.000Z");
        assert_eq!(param_type(&stamp), Type::UNKNOWN);
        assert_eq!(
            param_type(&Value::DateTime(
                chrono::DateTime::parse_from_rfc3339("2020-01-02T03:04:05.000Z")
                    .unwrap()
                    .with_timezone(&chrono::Utc)
            )),
            Type::TIMESTAMPTZ
        );
        assert_eq!(param_type(&Value::from("ada")), Type::UNKNOWN);
        assert!(matches!(
            BinaryConverter.convert(&Value::from("AQ==")).unwrap(),
            Value::Binary(_)
        ));
        assert_eq!(param_type(&Value::from("AA==")), Type::UNKNOWN);
        assert_eq!(param_type(&Value::Binary(vec![0])), Type::BYTEA);
        assert_eq!(param_type(&Value::Int(1)), Type::INT4);
        assert_eq!(param_type(&Value::Int(i64::MAX)), Type::INT8);
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
        assert_eq!(
            crate::connection::connect_url("postgres://db/app?schema=s"),
            "postgres://db/app"
        );
        assert_eq!(
            crate::connection::connect_url(
                "postgres://ds:ds@127.0.0.1:15432/datasources?schema=ds_x"
            ),
            "postgres://ds:ds@127.0.0.1:15432/datasources"
        );
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
