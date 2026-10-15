use super::identity_insert_statement::IdentityInsertStatement;
use super::insert_statement::InsertStatement;

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
                .map(|field| field.value.wire_text())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
