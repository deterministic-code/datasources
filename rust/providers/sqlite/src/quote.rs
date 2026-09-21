pub struct SqliteQuote;

impl SqliteQuote {
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
        DeleteStatement, InsertStatement, Join, Operator, Parameter, QueryStatement,
        StoredProcedureStatement, Text, UpdateStatement, WhereClause,
    };

    #[test]
    fn quotes_and_qualifies() {
        assert_eq!(SqliteQuote::quote("user"), "\"user\"");
        assert_eq!(SqliteQuote::quote("a\"b"), "\"a\"\"b\"");
        assert_eq!(SqliteQuote::qualified("users.id"), "\"users\".\"id\"");
    }

    #[test]
    fn quotes_tables_in_dml() {
        let where_clause = WhereClause::new(Operator::equals("id", 1_i64));
        let fields = [
            Parameter::new("id", 1_i64),
            Parameter::new("username", "alice"),
        ];
        let id = SqliteQuote::quote("id");
        let username = SqliteQuote::quote("username");
        assert_eq!(
            QueryStatement::new(
                SqliteQuote::quote("user"),
                &[&id, &username],
                Some(&where_clause),
                &[],
            )
            .get_text(),
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            InsertStatement::new(SqliteQuote::quote("user"), &fields).get_text(),
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)"
        );
        assert_eq!(
            UpdateStatement::new(SqliteQuote::quote("user"), &fields, Some(&where_clause)).get_text(),
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id"
        );
        assert_eq!(
            DeleteStatement::new(SqliteQuote::quote("user"), Some(&where_clause)).get_text(),
            "DELETE FROM \"user\" WHERE id = $id"
        );
        assert_eq!(
            StoredProcedureStatement::new(SqliteQuote::quote("save"), &fields).get_text(),
            "CALL \"save\"($id, $username)"
        );
        let users_id = SqliteQuote::qualified("users.id");
        let orders_total = SqliteQuote::qualified("orders.total");
        assert_eq!(
            QueryStatement::new(
                SqliteQuote::quote("users"),
                &[&users_id, &orders_total],
                Some(&WhereClause::new(Operator::equals("users.id", 1_i64))),
                &[
                    Join::inner(
                        SqliteQuote::qualified("orders"),
                        SqliteQuote::qualified("users.id"),
                        SqliteQuote::qualified("orders.user_id"),
                    ),
                    Join::left(
                        SqliteQuote::qualified("payments"),
                        SqliteQuote::qualified("orders.id"),
                        SqliteQuote::qualified("payments.order_id"),
                    ),
                ],
            )
            .get_text(),
            "SELECT \"users\".\"id\", \"orders\".\"total\" FROM \"users\" INNER JOIN \"orders\" ON \"users\".\"id\" = \"orders\".\"user_id\" LEFT JOIN \"payments\" ON \"orders\".\"id\" = \"payments\".\"order_id\" WHERE users.id = $users.id"
        );
    }
}
