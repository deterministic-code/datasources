use crate::schema;
use datasources::{Command, Native, Result};

pub async fn apply(command: &dyn Command, provider: &str) -> Result<()> {
    for table in schema::tables(provider)? {
        command.execute_statement(&table, None).await?;
    }
    if provider == "mysql" {
        for sql in schema::mysql_procedures() {
            command.execute_native(&Native::new(sql), None).await?;
        }
    }
    Ok(())
}
