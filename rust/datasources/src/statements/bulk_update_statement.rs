use super::bulk_insert_statement::flatten_bulk_parameters;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;

pub struct BulkUpdateRow {
    pub fields: Vec<Parameter>,
    pub ids: Vec<Parameter>,
}

pub struct BulkUpdateStatement {
    table: String,
    set: Vec<String>,
    ids: Vec<String>,
    rows: Vec<BulkUpdateRow>,
}

impl BulkUpdateStatement {
    pub fn new(
        table: impl AsRef<str>,
        set: &[String],
        ids: &[String],
        rows: Vec<BulkUpdateRow>,
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            set: set.to_vec(),
            ids: ids.to_vec(),
            rows,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn set(&self) -> &[String] {
        &self.set
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn rows(&self) -> &[BulkUpdateRow] {
        &self.rows
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        let columns: Vec<String> = self.set.iter().chain(self.ids.iter()).cloned().collect();
        let rows = self
            .rows
            .iter()
            .map(|row| {
                row.fields
                    .iter()
                    .chain(row.ids.iter())
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        flatten_bulk_parameters(&columns, &rows)
    }
}

impl crate::ReadInput for BulkUpdateStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for BulkUpdateStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_bulk_update(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Update
    }
}
