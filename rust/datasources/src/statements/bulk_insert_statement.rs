use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;
use crate::value::Value;
use std::collections::HashMap;

pub fn flatten_bulk_parameters(columns: &[String], rows: &[Vec<Parameter>]) -> Vec<Parameter> {
    let mut parameters = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let by_name: HashMap<_, _> = row
            .iter()
            .map(|field| (field.name.clone(), field.value.clone()))
            .collect();
        for column in columns {
            parameters.push(Parameter::new(
                format!("{column}__{index}"),
                by_name.get(column).cloned().unwrap_or(Value::Null),
            ));
        }
    }
    parameters
}

pub struct BulkInsertStatement {
    table: String,
    columns: Vec<String>,
    rows: Vec<Vec<Parameter>>,
}

impl BulkInsertStatement {
    pub fn new(table: impl AsRef<str>, columns: &[String], rows: Vec<Vec<Parameter>>) -> Self {
        Self {
            table: table.as_ref().to_string(),
            columns: columns.to_vec(),
            rows,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn rows(&self) -> &[Vec<Parameter>] {
        &self.rows
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        flatten_bulk_parameters(&self.columns, &self.rows)
    }
}

impl crate::ReadInput for BulkInsertStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for BulkInsertStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_bulk_insert(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Insert
    }
}
