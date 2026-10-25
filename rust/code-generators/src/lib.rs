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
mod mapping;
mod nested_select_types;
mod procedure_fields;
mod procedure_spec;
mod procedure_specs;
mod schema;
mod schema_diff;
mod seed_generator;
mod spec;
mod sql_dialect;
mod sql_generate_driver;
mod sql_keywords;
mod table_statements;
mod version_migrations;
mod write_inclusion;
mod yaml_cursor;

pub use generate_entry::GenerateEntry;
pub use generate_sql::generate_sql;
pub use nested_select_types::from_spec as nested_select_from_spec;
pub use generate_stored_procedures::generate_stored_procedures;
pub use sql_dialect::register_dialect;
pub use sql_generate_driver::SqlGenerateDriver;

use std::collections::HashMap;

use datasources::Result;

pub fn schema_sql(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
) -> Result<String> {
    let spec = spec::parse(files)?;
    let types = schema::from_spec(&spec);
    let mapping = mapping::SqlMapping::new(casing::PackCasing::of(settings)?, &types);
    let live = schema::build_live(&types, &mapping);
    let mut sql = String::new();
    for table in &live {
        sql.push_str(&sql_dialect::render_sql(
            dialect,
            &table_statements::create_table(dialect, table, &mapping)?,
        )?);
        sql.push('\n');
        for index in &table.indexes {
            let index_sql = sql_dialect::render_sql(
                dialect,
                &table_statements::create_index(dialect, &mapping, table, index)?,
            )?;
            if !index_sql.is_empty() {
                sql.push_str(&index_sql);
                sql.push('\n');
            }
        }
    }
    Ok(sql)
}
