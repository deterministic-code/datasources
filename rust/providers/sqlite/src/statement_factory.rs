use crate::create_table_statement_renderer::SqliteCreateTableStatementRenderer;
use datasources::{
    CreateTableStatement, CreateTableStatementRenderer, InsertedIdStatement, SelectStatement,
    Statement, Text,
};

pub struct SqliteStatementFactory;

pub enum Transformed<'a> {
    Same(&'a dyn Statement),
    Create(CreateTableStatement),
    Select(SelectStatement),
}

impl Transformed<'_> {
    pub fn get_text(&self) -> String {
        match self {
            Self::Same(statement) => statement.get_text(),
            Self::Create(statement) => statement.get_text(),
            Self::Select(statement) => statement.get_text(),
        }
    }
}

impl SqliteStatementFactory {
    pub fn transform<'a>(&self, statement: &'a dyn Statement) -> Transformed<'a> {
        if let Some(inserted) = statement.as_any().downcast_ref::<InsertedIdStatement>() {
            return Transformed::Select(SelectStatement::text(InsertedIdStatement::select_sql(
                "sqlite",
                &inserted.column,
            )));
        }
        match statement.as_any().downcast_ref::<CreateTableStatement>() {
            Some(create) if create.table.is_some() => Transformed::Create(CreateTableStatement::text(
                SqliteCreateTableStatementRenderer.render(create),
            )),
            _ => Transformed::Same(statement),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SqliteStatementFactory, Transformed};
    use datasources::{
        Column, ColumnDefault, CreateTableStatement, DeleteStatement, ForeignKey, NotNull,
        PrimaryKey, ReferentialAction, Text,
    };

    #[test]
    fn transforms_create_table_and_passes_other_statements_through() {
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
        let factory = SqliteStatementFactory;
        let Transformed::Create(transformed) = factory.transform(&create) else {
            panic!("compiled create table");
        };
        assert!(transformed.table.is_none());
        assert_eq!(transformed.get_text(), create.get_text());
        assert!(matches!(
            factory.transform(&transformed),
            Transformed::Same(_)
        ));

        let raw = CreateTableStatement::text("x");
        assert!(matches!(factory.transform(&raw), Transformed::Same(_)));
        let delete = DeleteStatement::raw("DELETE FROM users", None);
        assert!(matches!(factory.transform(&delete), Transformed::Same(_)));
        let inserted = datasources::InsertedIdStatement::new("users", "id");
        assert_eq!(
            factory.transform(&inserted).get_text(),
            "SELECT last_insert_rowid() AS id"
        );
    }

    #[test]
    fn renders_canonical_types() {
        let columns = [Column::new("id", "int")
            .identity()
            .default(ColumnDefault::new("0", None))
            .not_null(NotNull::new(None))];
        let create = CreateTableStatement::new("users", &columns, &[]);
        let Transformed::Create(transformed) = SqliteStatementFactory.transform(&create) else {
            panic!("compiled create table");
        };
        assert_eq!(
            transformed.get_text(),
            "CREATE TABLE users (id INTEGER DEFAULT 0 NOT NULL)"
        );
        assert_eq!(
            create.get_text(),
            "CREATE TABLE users (id int DEFAULT 0 NOT NULL)"
        );
    }
}
