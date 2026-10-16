use crate::alter_table_statement_renderer::MysqlAlterTableStatementRenderer;
use crate::create_routine_statement_renderer::MysqlCreateRoutineStatementRenderer;
use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::MysqlCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::MysqlDropRoutineStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::MysqlSelectInsertedIdStatementTextRenderer;
use datasources::{
    AlterTableStatement, BulkUpdateStatement,
    BulkUpdateStatementTextRenderer, CreateRoutineStatement, CreateTableStatement,
    CreateTableStatementTextRenderer, CreateTriggerStatement, DropIndexStatement,
    DropRoutineStatement, SelectInsertedIdStatement, StatementTextRenderer, StatementVisitor,
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
        format!("DROP INDEX {} ON {}", statement.name(), statement.table())
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

    fn visit_create_routine(
        &self,
        statement: &CreateRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        MysqlCreateRoutineStatementRenderer.render(statement)
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
}

#[cfg(test)]
mod tests {
    use super::MysqlStatementRenderer;
    use crate::create_table_statement_renderer::MysqlCreateTableStatementRenderer;
    use datasources::{
        BulkUpdateRow, BulkUpdateStatement, Column, ColumnDefault, CreateTableStatement,
        CreateTableStatementTextRenderer, DefaultStatementTextRenderer, NotNull, Parameter,
        SelectInsertedIdStatement, StatementTextRenderer,
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
}
