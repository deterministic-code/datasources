pub struct MysqlQuote;

impl MysqlQuote {
    pub fn quote(id: &str) -> String {
        format!("`{}`", id.replace('`', "``"))
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
        assert_eq!(MysqlQuote::quote("user"), "`user`");
        assert_eq!(MysqlQuote::quote("a`b"), "`a``b`");
        assert_eq!(MysqlQuote::qualified("users.id"), "`users`.`id`");
    }

    #[test]
    fn quotes_tables_in_dml() {
        let where_clause = WhereClause::new(Operator::equals("id", 1_i64));
        let fields = [
            Parameter::new("id", 1_i64),
            Parameter::new("username", "alice"),
        ];
        let id = MysqlQuote::quote("id");
        let username = MysqlQuote::quote("username");
        assert_eq!(
            QueryStatement::new(
                MysqlQuote::quote("user"),
                &[&id, &username],
                Some(&where_clause),
                &[],
            )
            .get_text(),
            "SELECT `id`, `username` FROM `user` WHERE id = $id"
        );
        assert_eq!(
            InsertStatement::new(MysqlQuote::quote("user"), &fields).get_text(),
            "INSERT INTO `user` (id, username) VALUES ($id, $username)"
        );
        assert_eq!(
            UpdateStatement::new(MysqlQuote::quote("user"), &fields, Some(&where_clause)).get_text(),
            "UPDATE `user` SET id = $id, username = $username WHERE id = $id"
        );
        assert_eq!(
            DeleteStatement::new(MysqlQuote::quote("user"), Some(&where_clause)).get_text(),
            "DELETE FROM `user` WHERE id = $id"
        );
        assert_eq!(
            ExecuteStoredProcedureStatement::new(MysqlQuote::quote("save"), &fields).get_text(),
            "CALL `save`($id, $username)"
        );
    }
}
