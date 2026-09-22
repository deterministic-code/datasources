use datasources::{Command, ExecuteStoredProcedureStatement, InsertStatement, Parameter, Result};

pub async fn execute(
    command: &dyn Command,
    provider: &str,
    table: &str,
    procedure: &str,
    insert: &[Parameter],
    call: &[Parameter],
) -> Result<String> {
    if provider == "mysql" {
        command
            .execute_statement(&ExecuteStoredProcedureStatement::new(procedure, call), Some(call))
            .await?;
    } else {
        command
            .execute_statement(&InsertStatement::new(table, insert), Some(insert))
            .await?;
    }
    Ok(String::new())
}
