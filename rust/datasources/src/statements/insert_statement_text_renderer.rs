use super::insert_statement::InsertStatement;

pub struct InsertStatementTextRenderer;

impl InsertStatementTextRenderer {
    pub fn render(&self, statement: &InsertStatement) -> String {
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
                .map(|field| format!("${}", field.name))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for InsertStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
