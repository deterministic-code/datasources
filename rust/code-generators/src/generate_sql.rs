use crate::casing::PackCasing;
use crate::custom_migrations;
use crate::generate_entry::GenerateEntry;
use crate::generate_procedures;
use crate::schema::{self, SqlTable};
use crate::seed_generator;
use crate::spec::{self, SeedRow};
use crate::sql_dialect;
use crate::table_statements;
use crate::version_migrations;
use datasources::Result;
use regex::Regex;
use std::collections::HashMap;

pub fn generate_sql(
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
    dialects: &[String],
) -> Result<Vec<GenerateEntry>> {
    let mut entries = Vec::new();
    for dialect in dialects {
        let key = sql_dialect::require(dialect)?;
        entries.extend(for_dialect(&key, files, settings)?);
    }
    Ok(entries)
}

fn for_dialect(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
) -> Result<Vec<GenerateEntry>> {
    let key = sql_dialect::require(dialect)?;
    let scoped = spec::files_for(files, &key);
    let casing = PackCasing::of(settings)?;
    let schema = schema_files(&key, &scoped, settings, &casing)?;
    let dir = settings.get("paths.deterministic").map(String::as_str);
    let mut entries = schema.0;
    entries.extend(custom_migrations::of(files, &key, dir)?);
    entries.extend(generate_procedures::for_dialect(dialect, &scoped, settings, schema.1 + 1)?);
    Ok(entries)
}

fn schema_files(
    key: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
    casing: &PackCasing,
) -> Result<(Vec<GenerateEntry>, i32)> {
    if !version_migrations::diff_paths(files).is_empty() {
        let versions = version_migrations::files(key, files, settings, casing)?;
        let count = (versions.len() / 2) as i32;
        let entries = versions
            .into_iter()
            .map(|entry| GenerateEntry::new(format!("{key}/migrations/{}", entry.filename), entry.content))
            .collect();
        return Ok((entries, count));
    }
    let parsed = spec::parse(files)?;
    let types = schema::from_spec(&parsed);
    let initial = initial(key, &types, &parsed.datasource_seeds, casing, settings)?;
    Ok((
        vec![
            GenerateEntry::new(format!("{key}/migrations/{}", initial.0.filename), initial.0.content),
            GenerateEntry::new(format!("{key}/migrations/{}", initial.1.filename), initial.1.content),
        ],
        1,
    ))
}

fn initial(
    language: &str,
    types: &[SqlTable],
    seeds_by_table: &HashMap<String, Vec<SeedRow>>,
    casing: &PackCasing,
    settings: &HashMap<String, String>,
) -> Result<(GenerateEntry, GenerateEntry)> {
    let dialect = sql_dialect::require(language)?;
    let naming = casing.strategy.clone();
    let live = schema::build_live(types, &naming);
    let seeds = seed_generator::sections(&dialect, &live, seeds_by_table, &naming)?;
    let separator = crate::join_batches::of(settings)?;
    Ok((
        GenerateEntry::new(
            "0001_initial_up.sql",
            finalize(&up_sql(&dialect, &live, &naming, &seeds, &separator)?, casing, &dialect)?,
        ),
        GenerateEntry::new(
            "0001_initial_down.sql",
            finalize(&down_sql(&dialect, &live, &separator)?, casing, &dialect)?,
        ),
    ))
}

fn flatten_table(dialect: &str, table: &SqlTable, naming: &datasources::StatementNaming, live: &[SqlTable]) -> Result<Vec<String>> {
    let mut parts = vec![sql_dialect::render_sql(
        dialect,
        &table_statements::create_table(table, naming, live)?,
    )?];
    for idx in &table.indexes {
        parts.push(sql_dialect::render_sql(
            dialect,
            &table_statements::create_index(naming, table, idx),
        )?);
    }
    if schema::has_audit_columns(table) {
        let overlay = schema::overlay_of(table);
        let pk = spec::pk_name(&table.as_type(), Some(&overlay));
        parts.push(sql_dialect::render_sql(
            dialect,
            &table_statements::create_trigger(table, naming, &pk),
        )?);
    }
    Ok(parts)
}

fn up_sql(
    dialect: &str,
    live: &[SqlTable],
    naming: &datasources::StatementNaming,
    seeds: &[String],
    separator: &str,
) -> Result<String> {
    let mut parts = Vec::new();
    for table in live {
        parts.extend(flatten_table(dialect, table, naming, live)?);
    }
    if !seeds.is_empty() {
        parts.push(format!(
            "-- Seed data\n{}",
            crate::join_batches::join(seeds, separator)
        ));
    }
    Ok(format!(
        "-- Generated initial migration for {dialect}\n\n-- Create added tables\n{}",
        crate::join_batches::join(&parts, separator)
    ))
}

fn down_sql(dialect: &str, live: &[SqlTable], separator: &str) -> Result<String> {
    let mut parts = Vec::new();
    for table in live.iter().rev() {
        parts.push(sql_dialect::render_sql(
            dialect,
            &table_statements::drop_table(&table.table_name),
        )?);
    }
    Ok(format!(
        "-- Rollback initial migration for {dialect}\n\n-- DROP tables (reverse dependency order)\n{}",
        crate::join_batches::join(&parts, separator)
    ))
}

fn finalize(text: &str, casing: &PackCasing, dialect: &str) -> Result<String> {
    let collapsed = Regex::new(r"\n{3,}").expect("nl").replace_all(text, "\n\n");
    let output = sql_dialect::schema_renderer(dialect, casing.strategy.clone())?.apply_keywords(&collapsed);
    Ok(if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    })
}
