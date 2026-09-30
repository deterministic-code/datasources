use super::statement::Statement;
use super::text::Text;
use crate::i_where::IWhere;
use crate::where_text::boxed;

pub struct DeleteStatement {
    text: Option<String>,
    table: Option<String>,
    where_clause: Option<Box<dyn IWhere>>,
}

impl DeleteStatement {
    pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self {
        Self {
            text: Some(text.into()),
            table: None,
            where_clause,
        }
    }

    pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self {
        Self {
            text: None,
            table: Some(table.as_ref().to_string()),
            where_clause: boxed(where_clause),
        }
    }

    pub fn table(&self) -> Option<&str> {
        self.table.as_deref()
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

impl Text for DeleteStatement {
    fn get_text(&self) -> String {
        let Some(table) = &self.table else {
            return self.text.clone().unwrap_or_default();
        };
        match &self.where_clause {
            Some(predicate) => format!("DELETE FROM {table} WHERE {}", predicate.get_text()),
            None => format!("DELETE FROM {table}"),
        }
    }
}

impl crate::ReadInput for DeleteStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for DeleteStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Delete
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}
