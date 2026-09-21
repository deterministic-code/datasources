use datasources::{
    Column, Join, Parameter, Result, StatementBuilder, TextCreateTableStatement,
    TextDeleteStatement, TextInsertStatement, TextQuery, TextStoredProcedureStatement,
    TextUpdateStatement, Where,
};

pub struct PostgresColumn {
    pub name: String,
    pub type_name: String,
    pub id: bool,
}

impl PostgresColumn {
    pub fn new(name: impl Into<String>, type_name: impl Into<String>, id: bool) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            id,
        }
    }
}

pub struct PostgresStatementBuilder;

impl PostgresStatementBuilder {
    pub fn quote(id: &str) -> String {
        format!("\"{}\"", id.replace('"', "\"\""))
    }

    pub fn quote_qualified(id: &str) -> String {
        id.split('.').map(Self::quote).collect::<Vec<_>>().join(".")
    }

    pub fn column_type(canonical: &str) -> Result<String> {
        Ok(match canonical {
            "int" => "INTEGER",
            "uuid" => "UUID",
            "string" => "TEXT",
            "bool" => "BOOLEAN",
            "decimal" => "NUMERIC",
            "float" => "DOUBLE PRECISION",
            "datetime" => "TIMESTAMPTZ",
            "binary" => "BYTEA",
            _ => {
                return Err(datasources::DatasourceError::message(format!(
                    "Unknown postgres type \"{canonical}\""
                )))
            }
        }
        .into())
    }

    pub fn create_table(
        table: &str,
        columns: &[PostgresColumn],
    ) -> Result<TextCreateTableStatement> {
        let mapped = columns
            .iter()
            .map(|column| {
                let affinity = Self::column_type(&column.type_name)?;
                Ok(Column::new(
                    Self::quote(&column.name),
                    if column.id {
                        format!("{affinity} PRIMARY KEY")
                    } else {
                        affinity
                    },
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(StatementBuilder::create_table(Self::quote(table), &mapped))
    }

    pub fn create_query(
        table: &str,
        fields: &[&str],
        where_clause: Option<&dyn Where>,
        joins: &[Join],
    ) -> TextQuery {
        let quoted: Vec<String> = fields
            .iter()
            .map(|field| Self::quote_qualified(field))
            .collect();
        let refs: Vec<&str> = quoted.iter().map(String::as_str).collect();
        let quoted_joins: Vec<Join> = joins
            .iter()
            .map(|join| Join {
                kind: join.kind,
                table: Self::quote_qualified(&join.table),
                left: Self::quote_qualified(&join.left),
                right: Self::quote_qualified(&join.right),
            })
            .collect();
        StatementBuilder::create_query(Self::quote(table), &refs, where_clause, &quoted_joins)
    }

    pub fn create_insert(table: &str, fields: &[Parameter]) -> TextInsertStatement {
        StatementBuilder::create_insert(Self::quote(table), fields)
    }

    pub fn create_stored_procedure(
        name: &str,
        parameters: &[Parameter],
    ) -> TextStoredProcedureStatement {
        StatementBuilder::create_stored_procedure(Self::quote(name), parameters)
    }

    pub fn create_update(
        table: &str,
        fields: &[Parameter],
        where_clause: Option<&dyn Where>,
    ) -> TextUpdateStatement {
        StatementBuilder::create_update(Self::quote(table), fields, where_clause)
    }

    pub fn create_delete(table: &str, where_clause: Option<&dyn Where>) -> TextDeleteStatement {
        StatementBuilder::create_delete(Self::quote(table), where_clause)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{Join, Operator, Statement, WhereStatement};

    fn user_columns() -> Vec<PostgresColumn> {
        vec![
            PostgresColumn::new("id", "int", true),
            PostgresColumn::new("uuid", "uuid", false),
            PostgresColumn::new("username", "string", false),
            PostgresColumn::new("is_enabled", "bool", false),
            PostgresColumn::new("balance", "decimal", false),
            PostgresColumn::new("location", "float", false),
            PostgresColumn::new("created", "datetime", false),
            PostgresColumn::new("version", "binary", false),
        ]
    }

    #[test]
    fn quotes_and_maps_types() {
        assert_eq!(PostgresStatementBuilder::quote("user"), "\"user\"");
        assert_eq!(PostgresStatementBuilder::quote("a\"b"), "\"a\"\"b\"");
        assert_eq!(
            PostgresStatementBuilder::column_type("int").unwrap(),
            "INTEGER"
        );
        let error = PostgresStatementBuilder::column_type("xml").unwrap_err();
        assert!(error.to_string().contains("Unknown postgres type \"xml\""));
    }

    #[test]
    fn creates_user_table() {
        assert_eq!(
            PostgresStatementBuilder::create_table("user", &user_columns())
                .unwrap()
                .get_text(),
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)"
        );
    }

    #[test]
    fn quotes_tables_in_dml() {
        let where_clause = WhereStatement::new(Operator::equals("id", 1_i64));
        let fields = [
            Parameter::new("id", 1_i64),
            Parameter::new("username", "alice"),
        ];
        assert_eq!(
            PostgresStatementBuilder::create_query(
                "user",
                &["id", "username"],
                Some(&where_clause),
                &[],
            )
            .get_text(),
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            PostgresStatementBuilder::create_insert("user", &fields).get_text(),
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)"
        );
        assert_eq!(
            PostgresStatementBuilder::create_update("user", &fields, Some(&where_clause))
                .get_text(),
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id"
        );
        assert_eq!(
            PostgresStatementBuilder::create_delete("user", Some(&where_clause)).get_text(),
            "DELETE FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            PostgresStatementBuilder::create_stored_procedure("save", &fields).get_text(),
            "CALL \"save\"($id, $username)"
        );
        assert_eq!(
            PostgresStatementBuilder::create_query(
                "users",
                &["users.id", "orders.total"],
                Some(&WhereStatement::new(Operator::equals("users.id", 1_i64))),
                &[
                    Join::inner("orders", "users.id", "orders.user_id"),
                    Join::left("payments", "orders.id", "payments.order_id"),
                ],
            )
            .get_text(),
            "SELECT \"users\".\"id\", \"orders\".\"total\" FROM \"users\" INNER JOIN \"orders\" ON \"users\".\"id\" = \"orders\".\"user_id\" LEFT JOIN \"payments\" ON \"orders\".\"id\" = \"payments\".\"order_id\" WHERE users.id = $users.id"
        );
    }
}
