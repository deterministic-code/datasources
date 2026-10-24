use super::contact_reads::contact_reads_all;
use datasources::{IWhere, SelectStatement, SelectStatementOptions, Where};

pub fn contacts_join_query(where_clause: Option<&Where>) -> SelectStatement {
    let nested = contact_reads_all();
    let field_refs: Vec<&str> = nested.select.fields.iter().map(String::as_str).collect();
    SelectStatement::new(SelectStatementOptions {
        joins: &nested.select.joins,
        where_clause: where_clause.map(|clause| clause as &dyn IWhere),
        ..SelectStatementOptions::new(&nested.select.table, &field_refs)
    })
}
