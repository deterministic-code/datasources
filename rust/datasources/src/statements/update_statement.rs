use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;
use crate::where_trait::Where;

pub struct UpdateStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl UpdateStatement {
    pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }

    pub fn new(table: impl AsRef<str>, fields: &[Parameter], where_clause: Option<&dyn Where>) -> Self {
        let sql = format!(
            "UPDATE {} SET {}",
            table.as_ref(),
            fields
                .iter()
                .map(|field| format!("{} = ${}", field.name, field.name))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let sql = match where_clause {
            Some(predicate) => format!("{sql} WHERE {}", predicate.get_text()),
            None => sql,
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

impl Text for UpdateStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for UpdateStatement {}
