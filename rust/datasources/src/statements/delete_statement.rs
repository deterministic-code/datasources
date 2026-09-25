use super::statement::Statement;
use super::text::Text;
use crate::i_where::IWhere;

pub struct DeleteStatement {
    text: String,
    where_clause: Option<Box<dyn IWhere>>,
}

impl DeleteStatement {
    pub fn raw(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }

    pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self {
        let sql = match where_clause {
            Some(predicate) => format!(
                "DELETE FROM {} WHERE {}",
                table.as_ref(),
                predicate.get_text()
            ),
            None => format!("DELETE FROM {}", table.as_ref()),
        };
        Self {
            text: sql,
            where_clause: where_clause.map(|predicate| {
                Box::new(crate::where_text::WhereText::new(predicate.get_text())) as Box<dyn IWhere>
            }),
        }
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

impl Text for DeleteStatement {
    fn get_text(&self) -> String {
        self.text.clone()
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
