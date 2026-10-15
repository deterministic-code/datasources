use super::identity_insert_statement::IdentityInsertStatement;
use super::insert_statement::InsertStatement;
use crate::value::Value;

fn sql(value: &Value) -> String {
    match value {
        Value::Null => "NULL".into(),
        Value::Text(text) => text.clone(),
        Value::Int(number) => number.to_string(),
        Value::Bool(true) => "TRUE".into(),
        Value::Bool(false) => "FALSE".into(),
        Value::Float(number) => number.to_string(),
        other => other.wire_text(),
    }
}

pub struct IdentityInsertStatementTextRenderer;

impl IdentityInsertStatementTextRenderer {
    pub fn render(&self, statement: &IdentityInsertStatement) -> String {
        statement
            .inserts()
            .iter()
            .map(|insert| self.insert(insert))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn insert(&self, statement: &InsertStatement) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({});",
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
                .map(|field| sql(&field.value))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
