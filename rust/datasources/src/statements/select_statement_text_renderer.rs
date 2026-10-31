use crate::builders::{Join, JoinKind};
use super::i_where::IWhere;
use super::select_field_aliases::apply_select_field_alias;
use super::select_statement::SelectStatement;
use super::where_text::WhereText;
use super::order_by::OrderBy;
use super::statement_text_renderer::StatementTextRenderer;

pub struct SelectStatementTextRenderer<'a> {
    text: &'a StatementTextRenderer,
}

impl<'a> SelectStatementTextRenderer<'a> {
    pub fn new(text: &'a StatementTextRenderer) -> Self {
        Self { text }
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
                    apply_select_field_alias(field, statement.aliases.get(index).and_then(|item| item.as_deref()))
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let distinct = if statement.distinct { "DISTINCT " } else { "" };
        format!(
            "SELECT {distinct}{fields} FROM {}{}{}{}{}{}{}{}",
            statement.table,
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
            names => format!(" GROUP BY {}", names.join(", ")),
        }
    }

    fn order_sql(&self, order: &[OrderBy]) -> String {
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
                JoinKind::Cross => format!(" CROSS JOIN {}", join.table),
                kind => format!(
                    " {} {} ON {}",
                    keyword(kind),
                    join.table,
                    self.text.render_where(&WhereText::new(
                        join.on.as_deref().expect("empty join"),
                    ))
                ),
            })
            .collect()
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
