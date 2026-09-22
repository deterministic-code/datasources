use super::statement::Statement;
use super::text::Text;
use crate::where_trait::Where;

pub struct DeleteStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl DeleteStatement {
    pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }

    pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn Where>) -> Self {
        let sql = match where_clause {
            Some(predicate) => format!("DELETE FROM {} WHERE {}", table.as_ref(), predicate.get_text()),
            None => format!("DELETE FROM {}", table.as_ref()),
        };
        Self {
            text: sql,
            where_clause: where_clause.map(|predicate| {
                Box::new(crate::text_where::TextWhere::new(predicate.get_text())) as Box<dyn Where>
            }),
        }
    }

    pub fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

impl Text for DeleteStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for DeleteStatement {
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Delete
    }
    fn predicate(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
