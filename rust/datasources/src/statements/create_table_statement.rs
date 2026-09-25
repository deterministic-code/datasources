use super::column_default::ColumnDefault;
use super::foreign_key::{ForeignKey, ReferentialAction};
use super::not_null::NotNull;
use super::statement::Statement;
use super::table_constraint::TableConstraint;
use super::text::Text;
use crate::builders::Column;

pub struct CreateTableStatement {
    text: String,
}

impl CreateTableStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(
        table: impl AsRef<str>,
        columns: &[Column],
        constraints: &[TableConstraint],
    ) -> Self {
        Self {
            text: format!(
                "CREATE TABLE {} ({})",
                table.as_ref(),
                parts(columns, constraints).join(", ")
            ),
        }
    }
}

impl Text for CreateTableStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for CreateTableStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateTableStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

fn parts(columns: &[Column], constraints: &[TableConstraint]) -> Vec<String> {
    let mut parts = columns.iter().map(column_sql).collect::<Vec<_>>();
    parts.extend(constraints.iter().map(constraint_sql));
    parts
}

fn column_sql(column: &Column) -> String {
    format!(
        "{} {}{}{}",
        column.name,
        column.type_name,
        default_sql(column.default.as_ref()),
        not_null_sql(column.not_null.as_ref())
    )
}

fn default_sql(value: Option<&ColumnDefault>) -> String {
    match value {
        None => String::new(),
        Some(ColumnDefault { value, name: None }) => format!(" DEFAULT {value}"),
        Some(ColumnDefault {
            value,
            name: Some(name),
        }) => format!(" CONSTRAINT {name} DEFAULT {value}"),
    }
}

fn not_null_sql(value: Option<&NotNull>) -> String {
    match value {
        None => String::new(),
        Some(NotNull { name: None }) => " NOT NULL".into(),
        Some(NotNull { name: Some(name) }) => format!(" CONSTRAINT {name} NOT NULL"),
    }
}

fn constraint_sql(constraint: &TableConstraint) -> String {
    match constraint {
        TableConstraint::PrimaryKey(pk) => {
            format!(
                "CONSTRAINT {} PRIMARY KEY ({})",
                pk.name,
                pk.columns.join(", ")
            )
        }
        TableConstraint::ForeignKey(fk) => foreign_key_sql(fk),
        TableConstraint::Unique(unique) => {
            format!(
                "CONSTRAINT {} UNIQUE ({})",
                unique.name,
                unique.columns.join(", ")
            )
        }
        TableConstraint::Check(check) => {
            format!("CONSTRAINT {} CHECK ({})", check.name, check.expression)
        }
    }
}

fn foreign_key_sql(fk: &ForeignKey) -> String {
    format!(
        "CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({}){}{}",
        fk.name,
        fk.columns.join(", "),
        fk.references_table,
        fk.references_columns.join(", "),
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
