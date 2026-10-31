use crate::alter_table_statement_renderer::PostgresAlterTableStatementRenderer;
use crate::stored_procedure_statement_renderer::PostgresStoredProcedureStatementRenderer;
use crate::create_table_statement_renderer::PostgresCreateTableStatementRenderer;
use crate::create_trigger_statement_renderer::PostgresCreateTriggerStatementRenderer;
use crate::drop_routine_statement_renderer::PostgresDropRoutineStatementRenderer;
use crate::identity_insert_statement_renderer::PostgresIdentityInsertStatementRenderer;
use crate::select_inserted_id_statement_text_renderer::PostgresSelectInsertedIdStatementTextRenderer;
use crate::sql_seed_statement_renderer::PostgresSqlSeedStatementRenderer;
use datasources::{
    AlterTableStatement, BulkInsertStatement, BulkUpdateStatement,
    BulkUpdateStatementTextRenderer, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement,
    CreateNestedReadStoredProcedureStatement, CreateReadStoredProcedureStatement, CreateTableStatement, CreateUpdateStoredProcedureStatement,
    CreateIndexStatement, CreateTableStatementTextRenderer, CreateTriggerStatement, DropIndexStatement,
    DropRoutineStatement, DropTableStatement, DropTriggerStatement, ForeignKeyStatement,
    IdentityInsertStatement, InsertStatement, SelectInsertedIdStatement,
    ProcedureSource, SqlSeedStatement, sql_procedure_statements_of, Statement, StatementTextRenderer, StatementVisitor, TypeConverter, Value,
};

pub struct PostgresStatementRenderer;

impl StatementVisitor for PostgresStatementRenderer {
    fn procedures(&self, source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
        sql_procedure_statements_of(source)
    }

    fn visit_create_table(
        &self,
        statement: &CreateTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresCreateTableStatementRenderer.render(statement)
    }

    fn visit_alter_table(
        &self,
        statement: &AlterTableStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresAlterTableStatementRenderer.render(statement)
    }

    fn visit_drop_trigger(
        &self,
        statement: &DropTriggerStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        format!(
            "DROP TRIGGER IF EXISTS {} ON {}",
            statement.name(),
            statement.table()
        )
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

    fn visit_drop_routine(
        &self,
        statement: &DropRoutineStatement,
        _text: &StatementTextRenderer,
    ) -> String {
        PostgresDropRoutineStatementRenderer.render(statement)
    }

    fn visit_insert(&self, statement: &InsertStatement, _text: &StatementTextRenderer) -> String {
        let insert = format!(
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
        );
        insert
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

    fn visit_create_create_stored_procedure(&self, statement: &CreateCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_create(statement)
    }
    fn visit_create_read_stored_procedure(&self, statement: &CreateReadStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_read(statement)
    }
    fn visit_create_nested_read_stored_procedure(&self, statement: &CreateNestedReadStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_nested_read(statement)
    }
    fn visit_create_update_stored_procedure(&self, statement: &CreateUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_update(statement)
    }
    fn visit_create_delete_stored_procedure(&self, statement: &CreateDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_delete(statement)
    }
    fn visit_create_bulk_create_stored_procedure(&self, statement: &CreateBulkCreateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_bulk_create(statement)
    }
    fn visit_create_bulk_update_stored_procedure(&self, statement: &CreateBulkUpdateStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_bulk_update(statement)
    }
    fn visit_create_bulk_delete_stored_procedure(&self, statement: &CreateBulkDeleteStoredProcedureStatement, _text: &StatementTextRenderer) -> String {
        PostgresStoredProcedureStatementRenderer.render_bulk_delete(statement)
    }

    fn visit_sql_seed(&self, statement: &SqlSeedStatement, _text: &StatementTextRenderer) -> String {
        PostgresSqlSeedStatementRenderer.render(statement)
    }

    fn visit_drop_table(&self, statement: &DropTableStatement, _text: &StatementTextRenderer) -> String {
        format!("DROP TABLE IF EXISTS {}", crate::PostgresDriver::quote(statement.table()))
    }

    fn visit_create_index(&self, statement: &CreateIndexStatement, _text: &StatementTextRenderer) -> String {
        format!(
            "CREATE {}INDEX {} ON {} ({})",
            if statement.unique() { "UNIQUE " } else { "" },
            crate::PostgresDriver::quote(statement.name()),
            crate::PostgresDriver::quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| crate::PostgresDriver::quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn visit_drop_index(&self, statement: &DropIndexStatement, _text: &StatementTextRenderer) -> String {
        format!("DROP INDEX IF EXISTS {}", crate::PostgresDriver::quote(statement.name()))
    }

    fn visit_foreign_key(&self, statement: &ForeignKeyStatement, _text: &StatementTextRenderer) -> String {
        let table = crate::PostgresDriver::quote(statement.table());
        let name = crate::PostgresDriver::quote(statement.name());
        if statement.kind() == "drop" {
            return format!("ALTER TABLE {table} DROP CONSTRAINT {name}");
        }
        let cols = statement
            .columns()
            .iter()
            .map(|column| crate::PostgresDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        let refs = statement
            .references_columns()
            .iter()
            .map(|column| crate::PostgresDriver::quote(column))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {} ({refs})",
            crate::PostgresDriver::quote(statement.references_table())
        )
    }
}

pub(crate) fn quote_identifiers(sql: &str) -> String {
    let chars: Vec<char> = sql.chars().collect();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            out.push('$');
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                out.push(chars[i]);
                i += 1;
            }
            continue;
        }
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            out.push(quote);
            i += 1;
            while i < chars.len() {
                out.push(chars[i]);
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let token: String = chars[start..i].iter().collect();
            if token.chars().any(|ch| ch.is_ascii_uppercase())
                && token.chars().any(|ch| ch.is_ascii_lowercase())
            {
                out.push('"');
                out.push_str(&token);
                out.push('"');
            } else {
                out.push_str(&token);
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
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
    if matches!(
        crate::converters::DateTimeConverter.convert(value),
        Ok(Value::DateTime(_))
    ) {
        return Some("datetime");
    }
    if matches!(
        crate::converters::BinaryConverter.convert(value),
        Ok(Value::Binary(_))
    ) {
        return Some("binary");
    }
    match value {
        Value::Null => None,
        Value::Bool(_) => Some("bool"),
        Value::Int(_) => Some("int"),
        Value::Float(_) => Some("float"),
        Value::Text(_) => Some("string"),
        Value::Uuid(_) => Some("uuid"),
        Value::Decimal(_) => Some("decimal"),
        Value::Binary(_) => Some("binary"),
        Value::DateTime(_) => Some("datetime"),
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
            "SELECT setval(pg_get_serial_sequence('users', 'id'), COALESCE((SELECT MAX(\"id\") FROM users), 1)) AS id"
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
                fields: vec![Parameter::new(
                    "created",
                    Value::DateTime(
                        chrono::DateTime::parse_from_rfc3339("2020-01-02T03:04:05.000Z")
                            .unwrap()
                            .with_timezone(&chrono::Utc),
                    ),
                )],
                ids: vec![Parameter::new("id", 1_i64)],
            }],
        );
        assert_eq!(
            renderer().render(&update),
            "UPDATE contacts SET created = data.created FROM (SELECT $created__0::TIMESTAMPTZ AS created, $id__0::INTEGER AS id) AS data WHERE contacts.id = data.id"
        );
    }

    #[test]
    fn renders_insert_without_prepending_setval() {
        let insert = InsertStatement::new("users", &[Parameter::new("name", "ada")]);
        assert_eq!(
            renderer().render(&insert),
            "INSERT INTO users (name) VALUES ($name::TEXT)"
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
