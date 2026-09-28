use crate::schema;
use datasources::{Command, Driver, Result, Text};

pub fn if_not_exists(sql: &str) -> String {
    match sql.strip_prefix("CREATE TABLE ") {
        Some(rest) => format!("CREATE TABLE IF NOT EXISTS {rest}"),
        None => sql.to_string(),
    }
}

pub async fn apply(command: &dyn Command, provider: &str, driver: &dyn Driver) -> Result<()> {
    for table in schema::tables(provider, driver)? {
        if provider == "sqlite" {
            command
                .execute(&if_not_exists(&table.get_text()), None)
                .await?;
        } else {
            command.execute_statement(&table, None).await?;
        }
    }
    if provider == "mysql" {
        for sql in schema::mysql_procedures() {
            command.execute(sql, None).await?;
        }
    }
    Ok(())
}
