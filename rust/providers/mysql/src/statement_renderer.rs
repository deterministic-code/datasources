use crate::alter_table_statement_renderer::MysqlAlterTableStatementRenderer;
use crate::stored_procedure_statement_renderer::MysqlStoredProcedureStatementRenderer;
use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::MysqlCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::MysqlDropRoutineStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::MysqlSelectInsertedIdStatementTextRenderer;
use crate::rebuild_table_statement_renderer::MysqlRebuildTableStatementRenderer;
use crate::sql_seed_statement_renderer::MysqlSqlSeedStatementRenderer;
use crate::MysqlDriver;
use datasources::{
    AlterTableStatement, BulkDeleteStatement, BulkDeleteStatementTextRenderer, BulkInsertStatement,
    BulkInsertStatementTextRenderer, BulkUpdateStatement,
    BulkUpdateStatementTextRenderer, CreateBulkCreateStoredProcedureStatement,
    DeleteStatement, DeleteStatementTextRenderer, InsertStatement, InsertStatementTextRenderer,
    SelectStatement, SelectStatementTextRenderer, UpdateStatement, UpdateStatementTextRenderer,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement,
    CreateNestedReadStoredProcedureStatement, CreateReadStoredProcedureStatement, CreateTableStatement,
    CreateTableStatementTextRenderer, CreateTriggerStatement, CreateUpdateStoredProcedureStatement,
    CreateIndexStatement, DropIndexStatement, DropRoutineStatement, DropTableStatement,
    DropTriggerStatement,
    ForeignKeyStatement, ProcedureSource, RebuildTableStatement, SelectInsertedIdStatement, SqlSeedStatement,
    sql_procedure_statements_of, Statement, StatementTextRenderer, StatementVisitor,
};

pub struct MysqlStatementRenderer;

impl StatementVisitor for MysqlStatementRenderer {
    fn procedures(&self, source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
        sql_procedure_statements_of(source)
    }

    fn visit_insert(&self, statement: &InsertStatement, _text: &StatementTextRenderer) -> String {
        let sql = InsertStatementTextRenderer::render_with(statement, MysqlDriver::quote, MysqlDriver::parameter);
        match statement.returning_column() {
            Some(column) => format!("{sql}; SELECT LAST_INSERT_ID() AS {column}"),
            None => sql,
        }
    }

    fn visit_update(&self, statement: &UpdateStatement, text: &StatementTextRenderer) -> String {
        UpdateStatementTextRenderer::new(text)
            .with_quote(MysqlDriver::quote, MysqlDriver::parameter)
            .render(statement)
    }

    fn visit_delete(&self, statement: &DeleteStatement, text: &StatementTextRenderer) -> String {
        DeleteStatementTextRenderer::new(text)
            .with_quote(MysqlDriver::quote)
            .render(statement)
    }

    fn visit_select(&self, statement: &SelectStatement, text: &StatementTextRenderer) -> String {
        SelectStatementTextRenderer::new(text)
            .with_quote(MysqlDriver::quote)
            .render(statement)
    }

    fn visit_bulk_insert(&self, statement: &BulkInsertStatement, _text: &StatementTextRenderer) -> String {
        let sql = BulkInsertStatementTextRenderer::render_with(
            statement,
            MysqlDriver::quote,
            MysqlDriver::parameter,
        );
        let Some(column) = statement.returning_column() else {
            return sql;
        };
        if statement.rows().is_empty() {
            return sql;
        }
        let selects = (0..statement.rows().len())
            .map(|index| {
                if index == 0 {
                    format!("SELECT LAST_INSERT_ID() AS {column}")
                } else {
                    format!("SELECT LAST_INSERT_ID() + {index} AS {column}")
                }
            })
            .collect::<Vec<_>>()
            .join(" UNION ALL ");
        format!("{sql}; {selects}")
    }

    fn visit_bulk_delete(&self, statement: &BulkDeleteStatement, _text: &StatementTextRenderer) -> String {
        BulkDeleteStatementTextRenderer::render_with(statement, MysqlDriver::quote, MysqlDriver::parameter)
    }

    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlCreateTableStatementRenderer.render(statement)
    }

    fn visit_alter_table(
        &self,
        statement: &AlterTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlAlterTableStatementRenderer.render(statement)
    }

    fn visit_drop_index(
        &self,
        statement: &DropIndexStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!(
            "DROP INDEX {} ON {}",
            crate::MysqlDriver::quote(&crate::identifier::fit(statement.name())),
            crate::MysqlDriver::quote(statement.table())
        )
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlSelectInsertedIdStatementTextRenderer.render(statement)
    }

    fn visit_create_trigger(
        &self,
        statement: &CreateTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlCreateTriggerStatementRenderer.render(statement)
    }

    fn visit_drop_trigger(
        &self,
        statement: &DropTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!(
            "DROP TRIGGER IF EXISTS {}",
            crate::MysqlDriver::quote(&crate::identifier::fit(statement.name()))
        )
    }

    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlDropRoutineStatementRenderer.render(statement)
    }

    fn visit_bulk_update(
        &self,
        statement: &BulkUpdateStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        BulkUpdateStatementTextRenderer.render_join(statement)
    }

    fn visit_create_create_stored_procedure(
        &self,
        statement: &CreateCreateStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_create(statement)
    }

    fn visit_create_read_stored_procedure(
        &self,
        statement: &CreateReadStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_read(statement)
    }

    fn visit_create_nested_read_stored_procedure(
        &self,
        statement: &CreateNestedReadStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_nested_read(statement)
    }

    fn visit_create_update_stored_procedure(
        &self,
        statement: &CreateUpdateStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_update(statement)
    }

    fn visit_create_delete_stored_procedure(
        &self,
        statement: &CreateDeleteStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_delete(statement)
    }

    fn visit_create_bulk_create_stored_procedure(
        &self,
        statement: &CreateBulkCreateStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_bulk_create(statement)
    }

    fn visit_create_bulk_update_stored_procedure(
        &self,
        statement: &CreateBulkUpdateStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_bulk_update(statement)
    }

    fn visit_create_bulk_delete_stored_procedure(
        &self,
        statement: &CreateBulkDeleteStoredProcedureStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlStoredProcedureStatementRenderer.render_bulk_delete(statement)
    }

    fn visit_sql_seed(&self, statement: &SqlSeedStatement, _text: &StatementTextRenderer) -> String {
        MysqlSqlSeedStatementRenderer.render(statement)
    }

    fn visit_drop_table(&self, statement: &DropTableStatement, _text: &StatementTextRenderer) -> String {
        format!("DROP TABLE IF EXISTS {}", crate::MysqlDriver::quote(statement.table()))
    }

    fn visit_create_index(&self, statement: &CreateIndexStatement, _text: &StatementTextRenderer) -> String {
        format!(
            "CREATE {}INDEX {} ON {} ({})",
            if statement.unique() { "UNIQUE " } else { "" },
            crate::MysqlDriver::quote(&crate::identifier::fit(statement.name())),
            crate::MysqlDriver::quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| crate::MysqlDriver::quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn visit_rebuild_table(&self, statement: &RebuildTableStatement, _text: &StatementTextRenderer) -> String {
        MysqlRebuildTableStatementRenderer.render(statement)
    }

    fn visit_foreign_key(&self, statement: &ForeignKeyStatement, _text: &StatementTextRenderer) -> String {
        let table = crate::MysqlDriver::quote(statement.table());
        let name = crate::MysqlDriver::quote(&crate::identifier::fit(statement.name()));
        if statement.kind() == "drop" {
            return format!("ALTER TABLE {table} DROP FOREIGN KEY {name}");
        }
        let cols = statement
            .columns()
            .iter()
            .map(|column| crate::MysqlDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        let refs = statement
            .references_columns()
            .iter()
            .map(|column| crate::MysqlDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {} ({refs})",
            crate::MysqlDriver::quote(statement.references_table())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MysqlStatementRenderer;
    use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
    use datasources::{
        BulkInsertStatement, BulkUpdateRow, BulkUpdateStatement, Column, ColumnDefault, CreateTableStatement,
        CreateTableStatementTextRenderer, DefaultStatementTextRenderer, NotNull, Parameter,
        InsertStatement, SelectInsertedIdStatement, StatementTextRenderer,
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
        let insert = InsertStatement::new("users", &[Parameter::new("name", "ada")]).returning("id");
        assert_eq!(
            renderer().render(&insert),
            "INSERT INTO users (name) VALUES (:name); SELECT LAST_INSERT_ID() AS id"
        );
        let bulk = BulkInsertStatement::new(
            "users",
            &["name".into()],
            vec![
                vec![Parameter::new("name", "ada")],
                vec![Parameter::new("name", "bea")],
            ],
        )
        .returning("id");
        assert_eq!(
            renderer().render(&bulk),
            "INSERT INTO users (name) VALUES (:name__0), (:name__1); SELECT LAST_INSERT_ID() AS id UNION ALL SELECT LAST_INSERT_ID() + 1 AS id"
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
    fn renders_named_default_without_constraint() {
        let create = CreateTableStatement::new(
            "genders",
            &[Column::new("uuid", "uuid")
                .default(ColumnDefault::new("(UUID())", Some("df_uuid")))
                .not_null(NotNull::new(Some("nn_uuid")))],
            &[],
        );
        let sql = renderer().render(&create);
        assert_eq!(
            sql,
            "CREATE TABLE genders (\n    uuid CHAR(36) DEFAULT (UUID()) NOT NULL\n)"
        );
        assert!(!sql.contains("CONSTRAINT"));
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
            "CREATE TABLE users (\n    id INT AUTO_INCREMENT DEFAULT 0 NOT NULL\n)"
        );
        assert_eq!(
            DefaultStatementTextRenderer::instance().render(&create),
            "CREATE TABLE users (\n    id int DEFAULT 0 NOT NULL\n)"
        );
    }

    #[test]
    fn renders_bulk_update_as_join() {
        let update = BulkUpdateStatement::new(
            "parents",
            &["label".into()],
            &["id".into()],
            vec![
                BulkUpdateRow {
                    fields: vec![Parameter::new("label", "a")],
                    ids: vec![Parameter::new("id", 1_i64)],
                },
                BulkUpdateRow {
                    fields: vec![Parameter::new("label", "b")],
                    ids: vec![Parameter::new("id", 2_i64)],
                },
            ],
        );
        assert_eq!(
            renderer().render(&update),
            "UPDATE parents JOIN (SELECT $label__0 AS label, $id__0 AS id UNION ALL SELECT $label__1 AS label, $id__1 AS id) AS data ON parents.id = data.id SET parents.label = data.label"
        );
    }

    #[test]
    fn renders_a_mysql_table_rebuild() {
        use datasources::{CreateIndexStatement, RebuildColumn, RebuildTableStatement};
        let rebuilt = renderer().render(&RebuildTableStatement::new(
            "users",
            CreateTableStatement::new(
                "users",
                &[Column::new("id", "INT"), Column::new("age", "INT")],
                &[],
            ),
            vec![
                RebuildColumn::copied("id", "id"),
                RebuildColumn::copied("age", "age").with_cast("INT"),
            ],
            vec![CreateIndexStatement::new("ix_users_age", "users", &["age"], false)],
            vec![],
        ));
        assert!(rebuilt.starts_with("SET FOREIGN_KEY_CHECKS = 0"));
        assert!(rebuilt.contains("users__new"));
        assert!(rebuilt.contains("CAST"));
        assert!(rebuilt.contains("SIGNED"));
        let index_at = rebuilt.find("ix_users_age").unwrap();
        let rename_at = rebuilt.find("RENAME TABLE").unwrap();
        let drop_at = rebuilt.find("users__old").unwrap();
        assert!(index_at < rename_at && rename_at < drop_at);
        assert!(rebuilt.ends_with("SET FOREIGN_KEY_CHECKS = 1"));
    }

    #[test]
    fn shortens_long_names_the_same_way_on_drop() {
        use datasources::{
            AlterAction, AlterTableStatement, CreateIndexStatement, CreateTriggerStatement,
            DropIndexStatement, DropTriggerStatement, ForeignKey, ForeignKeyStatement,
            UniqueConstraint,
        };
        let index = "game_journey_settings_game_journey_setting_category_type_id_index";
        let foreign_key = "game_journey_settings_game_journey_setting_category_type_id_foreign_key";
        let trigger = "trg_game_journey_setting_category_preference_option_values_updated_at";
        let unique = "game_journey_settings_external_reference_code_value_unique_constraint";
        let same = |sql: String, name: &str| {
            let fitted = crate::identifier::fit(name);
            assert!(fitted.chars().count() <= 64, "{fitted}");
            assert_ne!(fitted, name);
            assert!(sql.contains(&fitted), "{sql}");
            assert!(!sql.contains(name), "{sql}");
        };
        same(
            renderer().render(&CreateIndexStatement::new(
                index,
                "game_journey_settings",
                &["category_type_id"],
                false,
            )),
            index,
        );
        let unique_index = renderer().render(&CreateIndexStatement::new(
            index,
            "game_journey_settings",
            &["category_type_id"],
            true,
        ));
        same(unique_index.clone(), index);
        assert!(unique_index.starts_with("CREATE UNIQUE INDEX "));
        same(
            renderer().render(&DropIndexStatement::new(index, "game_journey_settings")),
            index,
        );
        same(
            renderer().render(&ForeignKeyStatement::new(
                "orders",
                foreign_key,
                vec!["category_type_id".into()],
                "categories",
                vec!["id".into()],
                "add",
            )),
            foreign_key,
        );
        same(
            renderer().render(&ForeignKeyStatement::new(
                "orders",
                foreign_key,
                vec![],
                "",
                vec![],
                "drop",
            )),
            foreign_key,
        );
        same(
            renderer().render(&CreateTriggerStatement::new(
                "game_journey_settings",
                trigger,
                "updated",
                "id",
            )),
            trigger,
        );
        same(
            renderer().render(&DropTriggerStatement::new(trigger, "game_journey_settings")),
            trigger,
        );
        same(
            renderer().render(&CreateTableStatement::new(
                "orders",
                &[Column::new("category_type_id", "INT")],
                &[
                    ForeignKey::new(
                        foreign_key,
                        &["category_type_id"],
                        "categories",
                        &["id"],
                        None,
                        None,
                    ),
                    UniqueConstraint::new(unique, &["category_type_id"]),
                ],
            )),
            foreign_key,
        );
        same(
            renderer().render(&AlterTableStatement::new(
                "orders",
                AlterAction::AddConstraint(UniqueConstraint::new(unique, &["category_type_id"])),
            )),
            unique,
        );
        same(
            renderer().render(&AlterTableStatement::new(
                "orders",
                AlterAction::DropConstraint {
                    name: unique.into(),
                },
            )),
            unique,
        );
    }
}
