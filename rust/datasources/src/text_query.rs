use crate::builders::{Join, JoinKind};
use crate::query::Query;
use crate::statements::statement::Statement;
use crate::statements::text::Text;
use crate::statements::{OrderBy, QueryStatementOptions};
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

    pub fn new(options: QueryStatementOptions<'_>) -> Self {
        let fields = if options.fields.is_empty() {
            "*".to_string()
        } else {
            options.fields.join(", ")
        };
        let sql = format!(
            "{}{}{}{}",
            with_where(
                format!(
                    "SELECT {fields} FROM {}{}",
                    options.table,
                    join_sql(options.joins)
                ),
                options.where_clause,
            ),
            group_by(options.group_by),
            order_by(options.order_by),
            limit_sql(options.limit),
        );
        match options.where_clause {
            Some(predicate) => {
                Self::with_where(sql, crate::text_where::TextWhere::new(predicate.get_text()))
            }
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
    fn kind(&self) -> crate::statements::StatementKind {
        crate::statements::StatementKind::Query
    }
    fn predicate(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

impl crate::ReadInput for QueryStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
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

fn group_by(columns: &[&str]) -> String {
    match columns {
        [] => String::new(),
        names => format!(" GROUP BY {}", names.join(", ")),
    }
}

fn order_by(order: &[OrderBy]) -> String {
    match order {
        [] => String::new(),
        items => format!(
            " ORDER BY {}",
            items
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn limit_sql(limit: Option<i64>) -> String {
    match limit {
        None => String::new(),
        Some(n) if n < 0 => panic!("negative limit"),
        Some(n) => format!(" LIMIT {n}"),
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
