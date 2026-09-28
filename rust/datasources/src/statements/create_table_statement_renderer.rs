use super::column_default::ColumnDefault;
use super::create_table_statement::CreateTableStatement;
use super::foreign_key::{ForeignKey, ReferentialAction};
use super::not_null::NotNull;
use super::table_constraint::TableConstraint;
use crate::builders::Column;

pub trait CreateTableStatementRenderer {
    fn quote(&self, name: &str) -> String {
        name.to_string()
    }

    fn suffix(&self) -> String {
        String::new()
    }

    fn type_name(&self, column: &Column) -> String {
        column.type_name.clone()
    }

    fn column_default(&self, column: &Column) -> String {
        self.write_default(column.default.as_ref())
    }

    fn write_default(&self, value: Option<&ColumnDefault>) -> String {
        match value {
            None => String::new(),
            Some(ColumnDefault { value, name: None }) => format!(" DEFAULT {value}"),
            Some(ColumnDefault {
                value,
                name: Some(name),
            }) => format!(" CONSTRAINT {} DEFAULT {value}", self.quote(name)),
        }
    }

    fn identity(&self, _column: &Column) -> String {
        String::new()
    }

    fn column(&self, column: &Column) -> String {
        format!(
            "{} {}{}{}{}",
            self.quote(&column.name),
            self.type_name(column),
            self.identity(column),
            self.column_default(column),
            not_null_sql(self, column.not_null.as_ref())
        )
    }

    fn constraint(&self, constraint: &TableConstraint) -> String {
        match constraint {
            TableConstraint::PrimaryKey(pk) => format!(
                "CONSTRAINT {} PRIMARY KEY ({})",
                self.quote(&pk.name),
                quoted(self, &pk.columns)
            ),
            TableConstraint::ForeignKey(fk) => foreign_key(self, fk),
            TableConstraint::Unique(unique) => format!(
                "CONSTRAINT {} UNIQUE ({})",
                self.quote(&unique.name),
                quoted(self, &unique.columns)
            ),
            TableConstraint::Check(check) => format!(
                "CONSTRAINT {} CHECK ({})",
                self.quote(&check.name),
                check.expression
            ),
        }
    }

    fn render(&self, statement: &CreateTableStatement) -> String {
        let mut parts = statement
            .columns
            .iter()
            .map(|column| self.column(column))
            .collect::<Vec<_>>();
        parts.extend(
            statement
                .constraints
                .iter()
                .map(|constraint| self.constraint(constraint)),
        );
        format!(
            "CREATE TABLE {} ({}){}",
            self.quote(statement.table.as_deref().unwrap_or("")),
            parts.join(", "),
            self.suffix()
        )
    }
}

struct DefaultRenderer;

impl CreateTableStatementRenderer for DefaultRenderer {}

pub(crate) fn render(statement: &CreateTableStatement) -> String {
    DefaultRenderer.render(statement)
}

fn not_null_sql<T: CreateTableStatementRenderer + ?Sized>(
    sql: &T,
    value: Option<&NotNull>,
) -> String {
    match value {
        None => String::new(),
        Some(NotNull { name: None }) => " NOT NULL".into(),
        Some(NotNull { name: Some(name) }) => format!(" CONSTRAINT {} NOT NULL", sql.quote(name)),
    }
}

fn quoted<T: CreateTableStatementRenderer + ?Sized>(sql: &T, names: &[String]) -> String {
    names
        .iter()
        .map(|name| sql.quote(name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn foreign_key<T: CreateTableStatementRenderer + ?Sized>(sql: &T, fk: &ForeignKey) -> String {
    format!(
        "CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({}){}{}",
        sql.quote(&fk.name),
        quoted(sql, &fk.columns),
        sql.quote(&fk.references_table),
        quoted(sql, &fk.references_columns),
        action_sql("DELETE", fk.on_delete),
        action_sql("UPDATE", fk.on_update)
    )
}

fn action_sql(kind: &str, action: Option<ReferentialAction>) -> String {
    match action {
        None => String::new(),
        Some(action) => format!(" ON {kind} {}", action_keyword(action)),
    }
}

fn action_keyword(action: ReferentialAction) -> &'static str {
    match action {
        ReferentialAction::Cascade => "CASCADE",
        ReferentialAction::SetNull => "SET NULL",
        ReferentialAction::SetDefault => "SET DEFAULT",
        ReferentialAction::Restrict => "RESTRICT",
        ReferentialAction::NoAction => "NO ACTION",
    }
}
