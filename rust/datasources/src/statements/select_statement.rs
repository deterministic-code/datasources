use crate::builders::Join;
use super::i_select_statement::ISelectStatement;
use super::i_where::IWhere;
use crate::statements::statement::Statement;
use crate::statements::{OrderBy, SelectStatementOptions};
use crate::{DefaultStatementTextRenderer, StatementTextRenderer};

pub struct SelectStatement {
    where_clause: Option<Box<dyn IWhere>>,
    pub distinct: bool,
    pub table: String,
    pub fields: Vec<String>,
    pub aliases: Vec<Option<String>>,
    pub joins: Vec<Join>,
    pub group_by: Vec<String>,
    pub having: Option<Box<dyn IWhere>>,
    pub order_by: Vec<OrderBy>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl SelectStatement {
    pub fn new(options: SelectStatementOptions<'_>) -> Self {
        Self {
            where_clause: options.where_clause.map(box_where),
            distinct: options.distinct,
            table: options.table.to_string(),
            fields: options
                .fields
                .iter()
                .map(|field| (*field).to_string())
                .collect(),
            aliases: options
                .aliases
                .iter()
                .map(|alias| alias.map(str::to_string))
                .collect(),
            joins: options.joins.to_vec(),
            group_by: options
                .group_by
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            having: options.having.map(box_where),
            order_by: options.order_by.to_vec(),
            limit: options.limit,
            offset: options.offset,
        }
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }

    pub fn having(&self) -> Option<&dyn IWhere> {
        self.having.as_deref()
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

impl Statement for SelectStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_select(self, text)
    }

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

fn box_where(predicate: &dyn IWhere) -> Box<dyn IWhere> {
    predicate
        .as_any()
        .downcast_ref::<crate::statements::Where>()
        .cloned()
        .map(|clause| Box::new(clause) as Box<dyn IWhere>)
        .unwrap_or_else(|| {
            Box::new(super::where_text::WhereText::new(
                DefaultStatementTextRenderer::instance().render_where(predicate),
            ))
        })
}
