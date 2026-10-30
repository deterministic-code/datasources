use crate::alter_table_statement_renderer::SqliteAlterTableStatementRenderer;
use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqliteCreateTriggerStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::SqliteSelectInsertedIdStatementTextRenderer;
use crate::quoted_renderers::{
    SqliteCreateIndexStatementRenderer, SqliteDropIndexStatementRenderer,
    SqliteDropTableStatementRenderer, SqliteDropTriggerStatementRenderer,
    SqliteForeignKeyStatementRenderer,
};
use crate::sql_seed_statement_renderer::SqliteSqlSeedStatementRenderer;
use datasources::{
    AlterTableStatement, CreateIndexStatement, CreateTableStatement, CreateTableStatementTextRenderer,
    CreateTriggerStatement, DropIndexStatement, DropTableStatement, DropTriggerStatement,
    ForeignKeyStatement, SelectInsertedIdStatement, SqlSeedStatement, StatementTextRenderer,
    StatementVisitor,
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

    fn visit_alter_table(
        &self,
        statement: &AlterTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        SqliteAlterTableStatementRenderer.render(statement)
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

    fn visit_sql_seed(&self, statement: &SqlSeedStatement, _text: &StatementTextRenderer) -> String {
        SqliteSqlSeedStatementRenderer.render(statement)
    }

    fn visit_drop_table(&self, statement: &DropTableStatement, _text: &StatementTextRenderer) -> String {
        SqliteDropTableStatementRenderer.render(statement)
    }

    fn visit_create_index(&self, statement: &CreateIndexStatement, _text: &StatementTextRenderer) -> String {
        SqliteCreateIndexStatementRenderer.render(statement)
    }

    fn visit_drop_index(&self, statement: &DropIndexStatement, _text: &StatementTextRenderer) -> String {
        SqliteDropIndexStatementRenderer.render(statement)
    }

    fn visit_drop_trigger(&self, statement: &DropTriggerStatement, _text: &StatementTextRenderer) -> String {
        SqliteDropTriggerStatementRenderer.render(statement)
    }

    fn visit_foreign_key(&self, statement: &ForeignKeyStatement, _text: &StatementTextRenderer) -> String {
        SqliteForeignKeyStatementRenderer.render(statement)
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
            "CREATE TABLE \"users\" (\"id\" INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL)"
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
            "CREATE TABLE \"users\" (\"id\" INTEGER PRIMARY KEY AUTOINCREMENT)"
        );
    }

    #[test]
    fn renders_alter_add_column() {
        use datasources::{AlterAction, AlterTableStatement, NotNull};
        let alter = AlterTableStatement::new(
            "users",
            AlterAction::AddColumn(Column::new("email", "TEXT").not_null(NotNull::new(None))),
        );
        assert_eq!(
            renderer().render(&alter),
            "ALTER TABLE \"users\" ADD COLUMN \"email\" TEXT NOT NULL"
        );
    }

    #[test]
    #[should_panic(expected = "SQLite cannot ALTER COLUMN")]
    fn throws_on_alter_column() {
        use datasources::{AlterAction, AlterTableStatement};
        let alter = AlterTableStatement::new("users", AlterAction::AlterColumn(Column::new("age", "INTEGER")));
        renderer().render(&alter);
    }
}
