use datasources::{Command, DatasourceError, Operator, QueryStatement, QueryStatementOptions, Result, Value, Where, WhereClause};

pub async fn of(command: &dyn Command, table: &str, column: &str, value: &str) -> Result<i64> {
    let where_clause = WhereClause::new(Operator::equals(column, value));
    let query = QueryStatement::new(QueryStatementOptions {
        where_clause: Some(&where_clause),
        ..QueryStatementOptions::new(table, &["id"])
    });
    let parameters = where_clause.parameters();
    let reader = command
        .execute_reader_statement(&query, Some(&parameters))
        .await?;
    if !reader.next().await? {
        return Err(DatasourceError::message(format!("{table} '{value}' not found")));
    }
    match reader.current().and_then(|row| row.get("id").cloned()) {
        Some(Value::Int(number)) => Ok(number),
        _ => Err(DatasourceError::message(format!("{table} '{value}' not found"))),
    }
}
