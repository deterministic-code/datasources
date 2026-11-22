use crate::alter_table_statement_renderer::SqliteAlterTableStatementRenderer;
use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::SqliteCreateTriggerStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::SqliteSelectInsertedIdStatementTextRenderer;
use crate::quoted_renderers::{
    SqliteCreateIndexStatementRenderer, SqliteDropIndexStatementRenderer,
    SqliteDropTableStatementRenderer, SqliteDropTriggerStatementRenderer,
    SqliteForeignKeyStatementRenderer, SqliteRebuildTableStatementRenderer,
};
use crate::sql_seed_statement_renderer::SqliteSqlSeedStatementRenderer;
use datasources::{
    AlterTableStatement, BulkInsertStatement, BulkInsertStatementTextRenderer, CreateIndexStatement,
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement, DropIndexStatement,
    DropTableStatement, DropTriggerStatement, ForeignKeyStatement, InsertStatement, RebuildTableStatement,
    InsertStatementTextRenderer, ProcedureSource,
    SelectInsertedIdStatement, SqlSeedStatement, sql_procedure_statements_of, Statement,
    StatementTextRenderer, StatementVisitor,
};

pub struct SqliteStatementRenderer;

impl StatementVisitor for SqliteStatementRenderer {
    fn procedures(&self, source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
        sql_procedure_statements_of(source)
    }

    fn visit_insert(&self, statement: &InsertStatement, _text: &StatementTextRenderer) -> String {
        let sql = InsertStatementTextRenderer::default().render(statement);
        match statement.returning_column() {
            Some(column) => format!("{sql} RETURNING last_insert_rowid() AS {column}"),
            None => sql,
        }
    }

    fn visit_bulk_insert(
        &self,
        statement: &BulkInsertStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        let sql = BulkInsertStatementTextRenderer.render(statement);
        match statement.returning_column() {
            Some(column) => format!("{sql} RETURNING last_insert_rowid() AS {column}"),
            None => sql,
        }
    }

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

    fn visit_rebuild_table(&self, statement: &RebuildTableStatement, _text: &StatementTextRenderer) -> String {
        SqliteRebuildTableStatementRenderer.render(statement)
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteStatementRenderer;
    use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
    use datasources::{
        Column, ColumnDefault, CreateTableStatement, CreateTableStatementTextRenderer,
        BulkInsertStatement, DefaultStatementTextRenderer, ForeignKey, InsertStatement, NotNull,
        Parameter, PrimaryKey,
        ReferentialAction, SelectInsertedIdStatement, SelectStatement, SelectStatementOptions,
        StatementTextRenderer,
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
        let insert = InsertStatement::new("users", &[Parameter::new("name", "ada")]).returning("id");
        assert_eq!(
            statements.render(&insert),
            "INSERT INTO users (name) VALUES ($name) RETURNING last_insert_rowid() AS id"
        );
        let bulk = BulkInsertStatement::new(
            "users",
            &["name".into()],
            vec![vec![Parameter::new("name", "ada")]],
        )
        .returning("id");
        assert_eq!(
            statements.render(&bulk),
            "INSERT INTO users (name) VALUES ($name__0) RETURNING last_insert_rowid() AS id"
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
            "CREATE TABLE \"users\" (\n    \"id\" INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL\n)"
        );
        assert_eq!(
            DefaultStatementTextRenderer::instance().render(&create),
            "CREATE TABLE users (\n    id int DEFAULT 0 NOT NULL\n)"
        );

        let keyed = CreateTableStatement::new(
            "users",
            &[Column::new("id", "int").identity()],
            &[PrimaryKey::new("pk_users", &["id"])],
        );
        assert_eq!(
            renderer().render(&keyed),
            "CREATE TABLE \"users\" (\n    \"id\" INTEGER PRIMARY KEY AUTOINCREMENT\n)"
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

    #[test]
    fn renders_a_sqlite_table_rebuild() {
        use datasources::{CreateIndexStatement, CreateTriggerStatement, RebuildColumn, RebuildTableStatement};
        let rebuilt = renderer().render(&RebuildTableStatement::new(
            "users",
            CreateTableStatement::new(
                "users",
                &[Column::new("id", "INTEGER"), Column::new("age", "INTEGER")],
                &[],
            ),
            vec![
                RebuildColumn::copied("id", "id"),
                RebuildColumn::copied("age", "age").with_cast("INTEGER"),
            ],
            vec![CreateIndexStatement::new("ix_users_age", "users", &["age"], false)],
            vec![],
        ));
        assert!(rebuilt.contains("CREATE TABLE \"users__new\""));
        assert!(rebuilt.contains(
            "INSERT INTO \"users__new\" (\"id\", \"age\") SELECT \"id\", CAST(\"age\" AS INTEGER) FROM \"users\""
        ));
        let create_at = rebuilt.find("CREATE TABLE \"users__new\"").unwrap();
        let drop_at = rebuilt.find("DROP TABLE IF EXISTS \"users\"").unwrap();
        let rename_at = rebuilt.find("ALTER TABLE \"users__new\" RENAME TO \"users\"").unwrap();
        let index_at = rebuilt.find("CREATE INDEX \"ix_users_age\" ON \"users\" (\"age\")").unwrap();
        assert!(create_at < drop_at && drop_at < rename_at && rename_at < index_at);
        let copied = renderer().render(&RebuildTableStatement::new(
            "users",
            CreateTableStatement::new(
                "users",
                &[
                    Column::new("id", "INTEGER"),
                    Column::new("note", "TEXT"),
                    Column::new("flag", "INTEGER"),
                ],
                &[],
            ),
            vec![
                RebuildColumn::copied("id", "id"),
                RebuildColumn::null("note"),
                RebuildColumn::literal("flag", "0"),
            ],
            vec![CreateIndexStatement::new("ix_users_note", "users", &["note"], false)],
            vec![CreateTriggerStatement::new("users", "trg_users", "note", "id")],
        ));
        assert!(copied.contains("SELECT \"id\", NULL, 0 FROM \"users\""));
        assert!(copied.contains("trg_users"));
    }
}
