use crate::builders::Join;
use crate::i_where::IWhere;
use crate::statements::OrderBy;

pub struct QueryStatementOptions<'a> {
    pub table: &'a str,
    pub fields: &'a [&'a str],
    pub where_clause: Option<&'a dyn IWhere>,
    pub joins: &'a [Join],
    pub group_by: &'a [&'a str],
    pub order_by: &'a [OrderBy],
    pub limit: Option<i64>,
    pub distinct: bool,
    pub having: Option<&'a dyn IWhere>,
    pub offset: Option<i64>,
}

impl<'a> QueryStatementOptions<'a> {
    pub fn new(table: &'a str, fields: &'a [&'a str]) -> Self {
        Self {
            table,
            fields,
            where_clause: None,
            joins: &[],
            group_by: &[],
            order_by: &[],
            limit: None,
            distinct: false,
            having: None,
            offset: None,
        }
    }
}
