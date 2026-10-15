use crate::create_routine_statement_renderer::PostgresCreateRoutineStatementRenderer;
use crate::create_table_statement_renderer::PostgresCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::PostgresCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::PostgresDropRoutineStatementRenderer;
use crate::identity_insert_statement_renderer::PostgresIdentityInsertStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::PostgresSelectInsertedIdStatementTextRenderer;
use datasources::{
    BulkInsertStatement, BulkUpdateStatement, BulkUpdateStatementTextRenderer, CreateRoutineStatement,
    CreateTableStatement, CreateTableStatementTextRenderer, CreateTriggerStatement,
    DropRoutineStatement, IdentityInsertStatement, InsertStatement, SelectInsertedIdStatement,
    StatementTextRenderer, StatementVisitor, Value,
};

pub struct PostgresStatementRenderer;

impl StatementVisitor for PostgresStatementRenderer {
    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresCreateTableStatementRenderer.render(statement)
    }

    fn visit_select_inserted_id(
        &self,
        statement: &SelectInsertedIdStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresSelectInsertedIdStatementTextRenderer.render(statement)
    }

    fn visit_create_trigger(
        &self,
        statement: &CreateTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresCreateTriggerStatementRenderer.render(statement)
    }

    fn visit_identity_insert(
        &self,
        statement: &IdentityInsertStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresIdentityInsertStatementRenderer.render(statement)
    }

    fn visit_create_routine(
        &self,
        statement: &CreateRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresCreateRoutineStatementRenderer.render(statement)
    }

    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresDropRoutineStatementRenderer.render(statement)
    }

    fn visit_insert(&self, statement: &InsertStatement, _text: &StatementTextRenderer) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            statement.table(),
            statement
                .fields()
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            statement
                .fields()
                .iter()
                .map(|field| pg_named(&field.name, &field.value))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn visit_bulk_insert(
        &self,
        statement: &BulkInsertStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        let values = statement
            .rows()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let by_name: std::collections::HashMap<_, _> =
                    row.iter().map(|field| (field.name.as_str(), &field.value)).collect();
                format!(
                    "({})",
                    statement
                        .columns()
                        .iter()
                        .map(|column| pg_param(
                            column,
                            index,
                            by_name.get(column.as_str()).copied().unwrap_or(&Value::Null)
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES {values}",
            statement.table(),
            statement.columns().join(", ")
        )
    }

    fn visit_bulk_update(
        &self,
        statement: &BulkUpdateStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        BulkUpdateStatementTextRenderer.render_cells(statement, pg_cell)
    }
}

fn pg_named(column: &str, value: &Value) -> String {
    match crate::driver::column_sql(logical(value).unwrap_or_default()) {
        Some(sql) => format!("${column}::{sql}"),
        None => format!("${column}"),
    }
}

fn pg_param(column: &str, index: usize, value: &Value) -> String {
    match crate::driver::column_sql(logical(value).unwrap_or_default()) {
        Some(sql) => format!("${column}__{index}::{sql}"),
        None => format!("${column}__{index}"),
    }
}

fn pg_cell(column: &str, index: usize, value: &Value) -> String {
    format!("{} AS {column}", pg_param(column, index, value))
}

fn logical(value: &Value) -> Option<&'static str> {
    match value {
        Value::Null => None,
        Value::Bool(_) => Some("bool"),
        Value::Int(_) => Some("int"),
        Value::Float(_) => Some("float"),
        Value::Text(text) if crate::bind::is_rfc3339(text) => Some("datetime"),
        Value::Text(text) if crate::bind::is_base64(text) => Some("binary"),
        Value::Text(_) => Some("string"),
        Value::DateTime(_) => Some("datetime"),
        Value::Uuid(_) => Some("uuid"),
        Value::Decimal(_) => Some("decimal"),
        Value::Binary(_) => Some("binary"),
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresStatementRenderer;
    use crate::create_table_statement_renderer::PostgresCreateTableStatementRenderer;
    use datasources::{
        BulkInsertStatement, BulkUpdateRow, BulkUpdateStatement, Column, ColumnDefault,
        CreateTableStatement, CreateTableStatementTextRenderer, DefaultStatementTextRenderer,
        InsertStatement, NotNull, Parameter, SelectInsertedIdStatement, StatementTextRenderer,
        Value,
    };

    fn renderer() -> StatementTextRenderer {
        StatementTextRenderer::new(Some(Box::new(PostgresStatementRenderer)), None)
    }

    #[test]
    fn renders_inserted_id_and_create_table() {
        assert_eq!(
            renderer().render(&SelectInsertedIdStatement::new("users", "id")),
            "SELECT lastval() AS id"
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
            PostgresCreateTableStatementRenderer.render(&create)
        );
    }

    #[test]
    fn renders_identity() {
        let create = CreateTableStatement::new(
            "users",
            &[
                Column::new("id", "int").identity(),
                Column::new("is_enabled", "bool").default(ColumnDefault::new("1", None)),
                Column::new("is_disabled", "bool")
                    .default(ColumnDefault::new("0", Some("df_off"))),
                Column::new("amount", "decimal").default(ColumnDefault::new("1", None)),
                Column::new("flag", "bool").default(ColumnDefault::new("true", None)),
            ],
            &[],
        );
        assert_eq!(
            renderer().render(&create),
            "CREATE TABLE users (id INTEGER GENERATED BY DEFAULT AS IDENTITY, is_enabled BOOLEAN DEFAULT TRUE, is_disabled BOOLEAN CONSTRAINT df_off DEFAULT FALSE, amount NUMERIC DEFAULT 1, flag BOOLEAN DEFAULT true)"
        );
        assert_eq!(
            DefaultStatementTextRenderer::instance().render(&create),
            "CREATE TABLE users (id int, is_enabled bool DEFAULT 1, is_disabled bool CONSTRAINT df_off DEFAULT 0, amount decimal DEFAULT 1, flag bool DEFAULT true)"
        );
    }

    #[test]
    fn renders_bulk_update_with_column_types() {
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
            "UPDATE parents SET label = data.label FROM (SELECT $label__0::TEXT AS label, $id__0::INTEGER AS id UNION ALL SELECT $label__1::TEXT AS label, $id__1::INTEGER AS id) AS data WHERE parents.id = data.id"
        );
    }

    #[test]
    fn renders_bulk_update_iso_created_as_timestamptz() {
        let update = BulkUpdateStatement::new(
            "contacts",
            &["created".into()],
            &["id".into()],
            vec![BulkUpdateRow {
                fields: vec![Parameter::new("created", "2020-01-02T03:04:05.000Z")],
                ids: vec![Parameter::new("id", 1_i64)],
            }],
        );
        assert_eq!(
            renderer().render(&update),
            "UPDATE contacts SET created = data.created FROM (SELECT $created__0::TIMESTAMPTZ AS created, $id__0::INTEGER AS id) AS data WHERE contacts.id = data.id"
        );
    }

    #[test]
    fn renders_insert_binary_version_as_bytea() {
        let insert = InsertStatement::new(
            "contacts",
            &[
                Parameter::new("version", Value::Binary(vec![1])),
                Parameter::new("id", 1_i64),
            ],
        );
        assert_eq!(
            renderer().render(&insert),
            "INSERT INTO contacts (version, id) VALUES ($version::BYTEA, $id::INTEGER)"
        );
    }

    #[test]
    fn renders_bulk_insert_binary_version_as_bytea() {
        let insert = BulkInsertStatement::new(
            "contacts",
            &["version".into(), "id".into()],
            vec![vec![
                Parameter::new("version", Value::Binary(vec![1])),
                Parameter::new("id", 1_i64),
            ]],
        );
        assert_eq!(
            renderer().render(&insert),
            "INSERT INTO contacts (version, id) VALUES ($version__0::BYTEA, $id__0::INTEGER)"
        );
    }

    #[test]
    fn renders_bulk_update_base64_version_as_bytea() {
        let update = BulkUpdateStatement::new(
            "contacts",
            &["version".into()],
            &["id".into()],
            vec![BulkUpdateRow {
                fields: vec![Parameter::new("version", "AA==")],
                ids: vec![Parameter::new("id", 1_i64)],
            }],
        );
        assert_eq!(
            renderer().render(&update),
            "UPDATE contacts SET version = data.version FROM (SELECT $version__0::BYTEA AS version, $id__0::INTEGER AS id) AS data WHERE contacts.id = data.id"
        );
    }
}
