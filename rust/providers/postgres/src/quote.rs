pub struct PostgresQuote;

impl PostgresQuote {
    pub fn quote(id: &str) -> String {
        format!("\"{}\"", id.replace('"', "\"\""))
    }

    pub fn qualified(id: &str) -> String {
        id.split('.').map(Self::quote).collect::<Vec<_>>().join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{
        DeleteStatement, InsertStatement, Operator, Parameter, QueryStatement,
        ExecuteStoredProcedureStatement, Text, UpdateStatement, WhereClause,
    };

    #[test]
    fn quotes_and_qualifies() {
        assert_eq!(PostgresQuote::quote("user"), "\"user\"");
        assert_eq!(PostgresQuote::quote("a\"b"), "\"a\"\"b\"");
        assert_eq!(PostgresQuote::qualified("users.id"), "\"users\".\"id\"");
    }

    #[test]
    fn quotes_tables_in_dml() {
        let where_clause = WhereClause::new(Operator::equals("id", 1_i64));
        let fields = [
            Parameter::new("id", 1_i64),
            Parameter::new("username", "alice"),
        ];
        let id = PostgresQuote::quote("id");
        let username = PostgresQuote::quote("username");
        assert_eq!(
            QueryStatement::new(
                PostgresQuote::quote("user"),
                &[&id, &username],
                Some(&where_clause),
                &[],
            )
            .get_text(),
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            InsertStatement::new(PostgresQuote::quote("user"), &fields).get_text(),
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)"
        );
        assert_eq!(
            UpdateStatement::new(PostgresQuote::quote("user"), &fields, Some(&where_clause)).get_text(),
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id"
        );
        assert_eq!(
            DeleteStatement::new(PostgresQuote::quote("user"), Some(&where_clause)).get_text(),
            "DELETE FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            ExecuteStoredProcedureStatement::new(PostgresQuote::quote("save"), &fields).get_text(),
            "CALL \"save\"($id, $username)"
        );
    }
}
