use crate::builders::{Join, JoinKind};
use super::i_where::IWhere;
use super::select_field_aliases::{as_index, segments};
use super::select_statement::SelectStatement;
use super::where_text::WhereText;
use super::order_by::{OrderBy, SortDirection};
use super::statement_text_renderer::StatementTextRenderer;

pub struct SelectStatementTextRenderer<'a> {
    text: &'a StatementTextRenderer,
    quote: fn(&str) -> String,
}

impl<'a> SelectStatementTextRenderer<'a> {
    pub fn new(text: &'a StatementTextRenderer) -> Self {
        Self {
            text,
            quote: |name| name.to_string(),
        }
    }

    pub fn with_quote(mut self, quote: fn(&str) -> String) -> Self {
        self.quote = quote;
        self
    }

    pub fn render(&self, statement: &SelectStatement) -> String {
        let fields = if statement.fields.is_empty() {
            "*".to_string()
        } else {
            statement
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    self.field_sql(field, statement.aliases.get(index).and_then(|item| item.as_deref()))
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let distinct = if statement.distinct { "DISTINCT " } else { "" };
        format!(
            "SELECT {distinct}{fields} FROM {}{}{}{}{}{}{}{}",
            self.quote(&statement.table),
            self.join_sql(&statement.joins),
            self.where_sql(statement.where_clause()),
            self.group_sql(&statement.group_by),
            self.having_sql(statement.having()),
            self.order_sql(&statement.order_by),
            self.limit(statement.limit),
            self.offset(statement.offset),
        )
    }

    fn where_sql(&self, where_clause: Option<&dyn IWhere>) -> String {
        match where_clause {
            Some(predicate) => format!(" WHERE {}", self.text.render_where(predicate)),
            None => String::new(),
        }
    }

    fn having_sql(&self, having: Option<&dyn IWhere>) -> String {
        match having {
            Some(predicate) => format!(" HAVING {}", self.text.render_where(predicate)),
            None => String::new(),
        }
    }

    fn group_sql(&self, columns: &[String]) -> String {
        match columns {
            [] => String::new(),
            names => format!(
                " GROUP BY {}",
                names
                    .iter()
                    .map(|name| self.quote_path(name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn order_sql(&self, order: &[OrderBy]) -> String {
        match order {
            [] => String::new(),
            items => format!(
                " ORDER BY {}",
                items
                    .iter()
                    .map(|item| {
                        format!(
                            "{} {}",
                            self.quote_path(&item.column),
                            if item.direction == SortDirection::Desc {
                                "DESC"
                            } else {
                                "ASC"
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn limit(&self, limit: Option<i64>) -> String {
        self.bound("LIMIT", limit)
    }

    fn offset(&self, offset: Option<i64>) -> String {
        self.bound("OFFSET", offset)
    }

    fn bound(&self, keyword: &str, value: Option<i64>) -> String {
        match value {
            None => String::new(),
            Some(n) if n < 0 => panic!("negative {}", keyword.to_lowercase()),
            Some(n) => format!(" {keyword} {n}"),
        }
    }

    fn join_sql(&self, joins: &[Join]) -> String {
        joins
            .iter()
            .map(|join| match join.kind {
                JoinKind::Cross => format!(" CROSS JOIN {}", self.quote(&join.table)),
                kind => format!(
                    " {} {} ON {}",
                    keyword(kind),
                    self.quote(&join.table),
                    self.text.render_where(&WhereText::new(
                        join.on.as_deref().expect("empty join"),
                    ))
                ),
            })
            .collect()
    }

    pub fn quote(&self, name: &str) -> String {
        (self.quote)(name)
    }

    fn quote_path(&self, name: &str) -> String {
        if name.contains('(') {
            name.to_string()
        } else {
            segments(name)
                .into_iter()
                .map(|part| self.quote(part))
                .collect::<Vec<_>>()
                .join(".")
        }
    }

    fn field_sql(&self, field: &str, alias: Option<&str>) -> String {
        let rendered = if as_index(field).is_some() || field.contains('(') {
            field.to_string()
        } else {
            self.quote_path(field)
        };
        match alias {
            Some(name) => format!("{} AS {}", rendered, self.quote(name)),
            None => rendered,
        }
    }
}

fn keyword(kind: JoinKind) -> &'static str {
    match kind {
        JoinKind::Inner => "INNER JOIN",
        JoinKind::Left => "LEFT JOIN",
        JoinKind::Right => "RIGHT JOIN",
        JoinKind::Full => "FULL JOIN",
        JoinKind::Cross => "CROSS JOIN",
    }
}
