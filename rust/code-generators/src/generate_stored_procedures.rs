use crate::generate_entry::GenerateEntry;
use crate::generate_procedures;
use crate::sql_dialect;
use datasources::Result;
use std::collections::HashMap;

pub fn generate_stored_procedures(
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
    dialects: &[String],
) -> Result<Vec<GenerateEntry>> {
    let mut entries = Vec::new();
    for dialect in dialects {
        let key = sql_dialect::require(dialect)?;
        entries.extend(generate_procedures::for_dialect(&key, files, settings, 2)?);
    }
    Ok(entries)
}
