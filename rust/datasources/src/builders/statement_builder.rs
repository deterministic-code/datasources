use super::column::Column;
use super::where_builder::WhereBuilder;
use crate::parameter::Parameter;
use crate::statement::Statement;
use crate::text_query::TextQuery;
use crate::statements::{
    TextCreateTableStatement, TextDeleteStatement, TextInsertStatement,
    TextStoredProcedureStatement, TextUpdateStatement,
};
use crate::where_trait::Where;

pub struct StatementBuilder;

impl StatementBuilder {
    pub fn create_query(
        table: impl AsRef<str>,
        fields: &[&str],
        where_clause: Option<WhereBuilder>,
    ) -> TextQuery {
        let sql = with_where(
            format!("SELECT {} FROM {}", select_list(fields), table.as_ref()),
            where_clause.as_ref(),
        );
        match where_clause {
            Some(predicate) => TextQuery::with_where(sql, predicate),
            None => TextQuery::new(sql),
        }
    }

    pub fn create_insert(table: impl AsRef<str>, fields: &[Parameter]) -> TextInsertStatement {
        TextInsertStatement::new(format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table.as_ref(),
            names(fields),
            placeholders(fields)
        ))
    }

    pub fn create_update(
        table: impl AsRef<str>,
        fields: &[Parameter],
        where_clause: Option<WhereBuilder>,
    ) -> TextUpdateStatement {
        let sql = format!("UPDATE {} SET {}", table.as_ref(), assignments(fields));
        TextUpdateStatement::new(
            with_where(sql, where_clause.as_ref()),
            where_clause.map(|w| Box::new(w) as Box<dyn Where>),
        )
    }

    pub fn create_delete(
        table: impl AsRef<str>,
        where_clause: Option<WhereBuilder>,
    ) -> TextDeleteStatement {
        let sql = format!("DELETE FROM {}", table.as_ref());
        TextDeleteStatement::new(
            with_where(sql, where_clause.as_ref()),
            where_clause.map(|w| Box::new(w) as Box<dyn Where>),
        )
    }

    pub fn create_table(table: impl AsRef<str>, columns: &[Column]) -> TextCreateTableStatement {
        let cols = columns
            .iter()
            .map(|column| format!("{} {}", column.name, column.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        TextCreateTableStatement::new(format!("CREATE TABLE {} ({})", table.as_ref(), cols))
    }

    pub fn create_stored_procedure(
        name: impl AsRef<str>,
        parameters: &[Parameter],
    ) -> TextStoredProcedureStatement {
        TextStoredProcedureStatement::new(format!(
            "CALL {}({})",
            name.as_ref(),
            placeholders(parameters)
        ))
    }
}

fn select_list(fields: &[&str]) -> String {
    if fields.is_empty() {
        "*".to_string()
    } else {
        fields.join(", ")
    }
}

fn names(fields: &[Parameter]) -> String {
    fields
        .iter()
        .map(|field| field.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn placeholders(fields: &[Parameter]) -> String {
    fields
        .iter()
        .map(|field| format!("${}", field.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn assignments(fields: &[Parameter]) -> String {
    fields
        .iter()
        .map(|field| format!("{} = ${}", field.name, field.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn with_where(sql: String, where_clause: Option<&WhereBuilder>) -> String {
    match where_clause {
        Some(predicate) => format!("{sql} WHERE {}", predicate.get_text()),
        None => sql,
    }
}
