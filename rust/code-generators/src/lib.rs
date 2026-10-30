#![allow(dead_code)]

mod by_fields;
mod by_groups;
mod casing;
mod compile_filter;
mod custom_migrations;
mod generate_entry;
mod generate_procedures;
mod generate_sql;
mod generate_stored_procedures;
mod join_batches;
mod nested_select_types;
mod procedure_fields;
mod schema;
mod schema_diff;
mod seed_generator;
mod spec;
mod sql_dialect;
mod table_statements;
mod version_migrations;
mod write_inclusion;
mod yaml_cursor;

pub use generate_entry::GenerateEntry;
pub use generate_sql::generate_sql;
pub use nested_select_types::from_spec as nested_select_from_spec;
pub use generate_stored_procedures::generate_stored_procedures;
pub use sql_dialect::register_dialect;

use std::collections::HashMap;

use datasources::Result;

pub fn schema_statements(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
) -> Result<Vec<String>> {
    let spec = spec::parse(files)?;
    let types = schema::from_spec(&spec);
    let naming = casing::PackCasing::of(settings)?.strategy;
    let live = schema::build_live(&types, &naming);
    let statements = schema_diff::of(dialect, &[], &naming, &live, &naming)?;
    statements
        .iter()
        .map(|statement| sql_dialect::render_sql(dialect, statement.as_statement()))
        .collect()
}

pub fn schema_sql(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
) -> Result<String> {
    Ok(schema_statements(dialect, files, settings)?.join("\n"))
}
