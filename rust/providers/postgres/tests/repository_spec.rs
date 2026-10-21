use super::command::PostgresCommand;
use super::connection::PostgresConnection;
use super::isolate;
use super::statement_renderer::PostgresStatementRenderer;
use datasources::{Connection, Statement, StatementTextRenderer};
use datasources_tests::RepositoryDialect;
use std::sync::Arc;

fn quote(id: &str) -> String {
    format!("\"{}\"", id.replace('"', "\"\""))
}

fn render(statement: &dyn Statement) -> String {
    StatementTextRenderer::new(Some(Box::new(PostgresStatementRenderer)), None).render(statement)
}

fn dialect() -> RepositoryDialect {
    RepositoryDialect {
        provider: "postgres",
        quote,
        field: super::converters::field,
        types: super::converters::row_type_converters,
        render: Some(render),
    }
}

#[tokio::test]
async fn user_crud() {
    for path in datasources_tests::files() {
        let isolated = isolate::create().await;
        let conn = Arc::new(PostgresConnection::new(Some(isolated.url.clone()), None));
        conn.open().await.unwrap();
        let command = Arc::new(PostgresCommand::new(conn.clone()));
        datasources_tests::run(&path, &dialect(), conn.clone(), command)
            .await
            .unwrap();
        conn.close().await.ok();
        isolated.drop().await;
    }
}
