mod alter_table_statement_renderer;
mod command;
mod connection;
mod create_table_statement_renderer;
mod create_trigger_statement_renderer;
mod drop_routine_statement_renderer;
mod driver;
mod export;
mod identity_insert_statement_renderer;
mod sql_generate;
mod statement_renderer;
mod stored_procedure_statement_renderer;

pub use driver::SqlServerDriver;

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, register, reset, BuiltDatasource, Datasource};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn ds() -> BuiltDatasource {
        BuiltDatasource::new("sqlserver", Arc::new(SqlServerDriver), None)
    }

    #[test]
    fn creates() {
        reset();
        register("sqlserver", Arc::new(SqlServerDriver));
        assert_eq!(
            create(Some(&HashMap::from([(
                "PROVIDER".into(),
                "sqlserver".into()
            )])))
            .unwrap()
            .provider(),
            "sqlserver"
        );
    }

    #[tokio::test]
    async fn creates_connection() {
        let connection = ds().create_connection().await.unwrap();
        assert!(connection.open().await.is_err());
    }

    #[tokio::test]
    async fn creates_command() {
        let ds = ds();
        let connection = ds.create_connection().await.unwrap();
        let command = ds.create_command(Some(connection)).await.unwrap();
        assert!(command.execute("", None).await.is_err());
        assert!(command.execute_reader("", None).await.is_err());
        assert!(ds.create_command(None).await.is_err());
    }
}
