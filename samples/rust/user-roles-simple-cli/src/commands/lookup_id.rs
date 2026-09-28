use datasources::{
    Command, DatasourceError, IWhere, Operator, Result, SelectStatement, SelectStatementOptions,
    Value, Where,
};

pub async fn of(command: &dyn Command, table: &str, column: &str, value: &str) -> Result<i64> {
    let where_clause = Where::new(Operator::equals(column, value));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new(table, &["id"])
    });
    let parameters = where_clause.parameters();
    let reader = command
        .execute_reader_statement(&query, Some(&parameters))
        .await?;
    if !reader.next().await? {
        return Err(DatasourceError::message(format!(
            "{table} '{value}' not found"
        )));
    }
    match reader.current().and_then(|row| row.get("id").cloned()) {
        Some(Value::Int(number)) => Ok(number),
        _ => Err(DatasourceError::message(format!(
            "{table} '{value}' not found"
        ))),
    }
}
