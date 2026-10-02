use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::MysqlSelectInsertedIdStatementTextRenderer;
use datasources::{
    CreateTableStatement, CreateTableStatementTextRenderer, SelectInsertedIdStatement,
    StatementTextRenderer, StatementVisitor,
};

pub struct MysqlStatementRenderer;

impl StatementVisitor for MysqlStatementRenderer {
    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlCreateTableStatementRenderer.render(statement)
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlSelectInsertedIdStatementTextRenderer.render(statement)
    }
}

#[cfg(test)]
mod tests {
    use super::MysqlStatementRenderer;
    use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
    use datasources::{
        Column, ColumnDefault, CreateTableStatement, CreateTableStatementTextRenderer,
        DefaultStatementTextRenderer, NotNull, SelectInsertedIdStatement, StatementTextRenderer,
    };

    fn renderer() -> StatementTextRenderer {
        StatementTextRenderer::new(Some(Box::new(MysqlStatementRenderer)), None)
    }

    #[test]
    fn renders_inserted_id_and_create_table() {
        assert_eq!(
            renderer().render(&SelectInsertedIdStatement::new("users", "id")),
            "SELECT LAST_INSERT_ID() AS id"
        );
        let create = CreateTableStatement::new(
            "orders",
            &[
                Column::new("id", "INTEGER")
                    .default(ColumnDefault::new("0", Some("df_id")))
                    .not_null(NotNull::new(Some("nn_id"))),
                Column::new("user_id", "INTEGER"),
            ],
            &[],
        );
        assert_eq!(
            renderer().render(&create),
            MysqlCreateTableStatementRenderer.render(&create)
        );
    }

    #[test]
    fn renders_identity() {
        let create = CreateTableStatement::new(
            "users",
            &[Column::new("id", "int")
                .identity()
                .default(ColumnDefault::new("0", None))
                .not_null(NotNull::new(None))],
            &[],
        );
        assert_eq!(
            renderer().render(&create),
            "CREATE TABLE users (id INT AUTO_INCREMENT DEFAULT 0 NOT NULL)"
        );
        assert_eq!(
            DefaultStatementTextRenderer::instance().render(&create),
            "CREATE TABLE users (id int DEFAULT 0 NOT NULL)"
        );
    }
}
