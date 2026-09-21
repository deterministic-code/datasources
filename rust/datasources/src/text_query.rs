use crate::builders::{Join, JoinKind};
use crate::query::Query;
use crate::statements::statement::Statement;
use crate::statements::text::Text;
use crate::where_trait::Where;

pub struct QueryStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl QueryStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl Where + 'static) -> Self {
        Self {
            text: text.into(),
            where_clause: Some(Box::new(where_clause)),
        }
    }

    pub fn new(
        table: impl AsRef<str>,
        fields: &[&str],
        where_clause: Option<&dyn Where>,
        joins: &[Join],
    ) -> Self {
        let sql = with_where(
            format!(
                "SELECT {} FROM {}{}",
                if fields.is_empty() {
                    "*".to_string()
                } else {
                    fields.join(", ")
                },
                table.as_ref(),
                join_sql(joins)
            ),
            where_clause,
        );
        match where_clause {
            Some(predicate) => Self::with_where(sql, crate::text_where::TextWhere::new(predicate.get_text())),
            None => Self::text(sql),
        }
    }
}

impl Text for QueryStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for QueryStatement {
    fn parameters(&self) -> Vec<crate::parameter::Parameter> {
        self.where_clause
            .as_ref()
            .map(|predicate| predicate.parameters())
            .unwrap_or_default()
    }
}

impl Query for QueryStatement {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

fn with_where(sql: String, where_clause: Option<&dyn Where>) -> String {
    match where_clause {
        Some(predicate) => format!("{sql} WHERE {}", predicate.get_text()),
        None => sql,
    }
}

fn join_sql(joins: &[Join]) -> String {
    joins
        .iter()
        .map(|join| {
            format!(
                " {} {} ON {} = {}",
                match join.kind {
                    JoinKind::Inner => "INNER JOIN",
                    JoinKind::Left => "LEFT JOIN",
                    JoinKind::Right => "RIGHT JOIN",
                },
                join.table,
                join.left,
                join.right
            )
        })
        .collect()
}
