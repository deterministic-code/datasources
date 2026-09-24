use crate::builders::Join;
use crate::statements::OrderBy;
use crate::where_trait::Where;

pub struct QueryStatementOptions<'a> {
    pub table: &'a str,
    pub fields: &'a [&'a str],
    pub where_clause: Option<&'a dyn Where>,
    pub joins: &'a [Join],
    pub group_by: &'a [&'a str],
    pub order_by: &'a [OrderBy],
    pub limit: Option<i64>,
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
        }
    }
}
