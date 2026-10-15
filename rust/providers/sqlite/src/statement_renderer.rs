use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqliteCreateTriggerStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::SqliteSelectInsertedIdStatementTextRenderer;
use datasources::{
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    SelectInsertedIdStatement, StatementTextRenderer, StatementVisitor,
};

pub struct SqliteStatementRenderer;

impl StatementVisitor for SqliteStatementRenderer {
    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqliteCreateTableStatementRenderer.render(statement)
    }

    fn visit_create_trigger(
        &self,
        statement: &CreateTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqliteCreateTriggerStatementRenderer.render(statement)
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqliteSelectInsertedIdStatementTextRenderer.render(statement)
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteStatementRenderer;
    use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
    use datasources::{
        Column, ColumnDefault, CreateTableStatement, CreateTableStatementTextRenderer,
        DefaultStatementTextRenderer, ForeignKey, NotNull, PrimaryKey, ReferentialAction,
        SelectInsertedIdStatement, SelectStatement, SelectStatementOptions, StatementTextRenderer,
    };

    fn renderer() -> StatementTextRenderer {
        StatementTextRenderer::new(Some(Box::new(SqliteStatementRenderer)), None)
    }

    #[test]
    fn renders_inserted_id_and_create_table() {
        let statements = renderer();
        assert_eq!(
            statements.render(&SelectInsertedIdStatement::new("users", "id")),
            "SELECT last_insert_rowid() AS id"
        );

        let columns = [
            Column::new("id", "INTEGER")
                .default(ColumnDefault::new("0", Some("df_id")))
                .not_null(NotNull::new(Some("nn_id"))),
            Column::new("user_id", "INTEGER"),
        ];
        let constraints = [
            PrimaryKey::new("pk_orders", &["id"]),
            ForeignKey::new(
                "fk_orders_user",
                &["user_id"],
                "users",
                &["id"],
                Some(ReferentialAction::Cascade),
                None,
            ),
        ];
        let create = CreateTableStatement::new("orders", &columns, &constraints);
        assert_eq!(
            statements.render(&create),
            SqliteCreateTableStatementRenderer.render(&create)
        );
        assert_eq!(
            statements.render(&SelectStatement::new(SelectStatementOptions::new(
                "users",
                &["id"]
            ))),
            "SELECT id FROM users"
        );
    }

    #[test]
    fn renders_canonical_types() {
        let columns = [Column::new("id", "int")
            .identity()
            .default(ColumnDefault::new("0", None))
            .not_null(NotNull::new(None))];
        let create = CreateTableStatement::new("users", &columns, &[]);
        assert_eq!(
            renderer().render(&create),
            "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL)"
        );
        assert_eq!(
            DefaultStatementTextRenderer::instance().render(&create),
            "CREATE TABLE users (id int DEFAULT 0 NOT NULL)"
        );

        let keyed = CreateTableStatement::new(
            "users",
            &[Column::new("id", "int").identity()],
            &[PrimaryKey::new("pk_users", &["id"])],
        );
        assert_eq!(
            renderer().render(&keyed),
            "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT)"
        );
    }
}
