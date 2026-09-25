use crate::builders::{Join, JoinKind};
use crate::i_select_statement::ISelectStatement;
use crate::i_where::IWhere;
use crate::statements::statement::Statement;
use crate::statements::text::Text;
use crate::statements::{OrderBy, SelectStatementOptions};

pub struct SelectStatement {
    text: String,
    where_clause: Option<Box<dyn IWhere>>,
    pub distinct: bool,
    pub table: String,
    pub fields: Vec<String>,
    pub joins: Vec<Join>,
    pub group_by: Vec<String>,
    pub having: Option<Box<dyn IWhere>>,
    pub order_by: Vec<OrderBy>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl SelectStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            where_clause: None,
            distinct: false,
            table: String::new(),
            fields: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn with_where(text: impl Into<String>, where_clause: impl IWhere + 'static) -> Self {
        Self {
            text: text.into(),
            where_clause: Some(Box::new(where_clause)),
            distinct: false,
            table: String::new(),
            fields: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn new(options: SelectStatementOptions<'_>) -> Self {
        let fields = if options.fields.is_empty() {
            "*".to_string()
        } else {
            options.fields.join(", ")
        };
        let distinct = if options.distinct { "DISTINCT " } else { "" };
        let sql = format!(
            "{}{}{}{}{}{}",
            with_where(
                format!(
                    "SELECT {distinct}{fields} FROM {}{}",
                    options.table,
                    join_sql(options.joins)
                ),
                options.where_clause,
            ),
            group_by(options.group_by),
            having_sql(options.having),
            order_by(options.order_by),
            bound("LIMIT", options.limit),
            bound("OFFSET", options.offset),
        );
        let mut statement = match options.where_clause {
            Some(predicate) => {
                Self::with_where(sql, crate::where_text::WhereText::new(predicate.get_text()))
            }
            None => Self::text(sql),
        };
        statement.distinct = options.distinct;
        statement.table = options.table.to_string();
        statement.fields = options
            .fields
            .iter()
            .map(|field| (*field).to_string())
            .collect();
        statement.joins = options.joins.to_vec();
        statement.group_by = options
            .group_by
            .iter()
            .map(|column| (*column).to_string())
            .collect();
        statement.having = options.having.map(|predicate| {
            Box::new(crate::where_text::WhereText::new(predicate.get_text())) as Box<dyn IWhere>
        });
        statement.order_by = options.order_by.to_vec();
        statement.limit = options.limit;
        statement.offset = options.offset;
        statement
    }

    pub fn replace_where(mut self, where_clause: impl IWhere + 'static) -> Self {
        self.where_clause = Some(Box::new(where_clause));
        self
    }

    pub fn replace_having(mut self, having: impl IWhere + 'static) -> Self {
        self.having = Some(Box::new(having));
        self
    }
}

impl Text for SelectStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for SelectStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> crate::statements::StatementKind {
        crate::statements::StatementKind::Select
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

impl crate::ReadInput for SelectStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl ISelectStatement for SelectStatement {
    fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

fn with_where(sql: String, where_clause: Option<&dyn IWhere>) -> String {
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

fn having_sql(having: Option<&dyn IWhere>) -> String {
    match having {
        Some(predicate) => format!(" HAVING {}", predicate.get_text()),
        None => String::new(),
    }
}

fn bound(keyword: &str, value: Option<i64>) -> String {
    match value {
        None => String::new(),
        Some(n) if n < 0 => panic!("negative {}", keyword.to_lowercase()),
        Some(n) => format!(" {keyword} {n}"),
    }
}

fn join_sql(joins: &[Join]) -> String {
    joins
        .iter()
        .map(|join| match join.kind {
            JoinKind::Cross => format!(" CROSS JOIN {}", join.table),
            kind => format!(
                " {} {} ON {}",
                join_word(kind),
                join.table,
                join.on.as_deref().expect("empty join")
            ),
        })
        .collect()
}

fn join_word(kind: JoinKind) -> &'static str {
    match kind {
        JoinKind::Inner => "INNER JOIN",
        JoinKind::Left => "LEFT JOIN",
        JoinKind::Right => "RIGHT JOIN",
        JoinKind::Full => "FULL JOIN",
        JoinKind::Cross => "CROSS JOIN",
    }
}
