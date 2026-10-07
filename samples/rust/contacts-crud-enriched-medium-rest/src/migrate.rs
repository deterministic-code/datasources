use crate::schema;
use datasources::{Command, Native, Result};

pub async fn apply(command: &dyn Command, provider: &str) -> Result<()> {
    if provider == "sqlite" {
        command
            .execute_native(&Native::new("PRAGMA foreign_keys = ON"), None)
            .await?;
    }
    for sql in schema::load_statements() {
        command.execute_native(&Native::new(sql), None).await?;
    }
    Ok(())
}
