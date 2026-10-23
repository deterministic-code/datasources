use crate::casing::PackCasing;
use crate::custom_migrations;
use crate::generate_entry::GenerateEntry;
use crate::generate_procedures;
use crate::mapping::SqlMapping;
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
    let initial = initial(key, &types, &parsed.datasource_seeds, casing)?;
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
) -> Result<(GenerateEntry, GenerateEntry)> {
    let dialect = sql_dialect::require(language)?;
    let mapping = SqlMapping::new(casing.clone(), types);
    let live = schema::build_live(types, &mapping);
    let seeds = seed_generator::sections(&dialect, &live, seeds_by_table, &mapping)?;
    Ok((
        GenerateEntry::new(
            "0001_initial_up.sql",
            finalize(&up_sql(&dialect, &live, &mapping, &seeds)?, casing),
        ),
        GenerateEntry::new("0001_initial_down.sql", finalize(&down_sql(&dialect, &live)?, casing)),
    ))
}

fn up_sql(
    dialect: &str,
    live: &[SqlTable],
    mapping: &SqlMapping,
    seeds: &[String],
) -> Result<String> {
    let mut body = format!("-- Generated initial migration for {dialect}\n\n-- Create added tables\n");
    for table in live {
        let mut indexes = Vec::new();
        for idx in &table.indexes {
            indexes.push(sql_dialect::render_sql(
                dialect,
                &table_statements::create_index(dialect, mapping, table, idx)?,
            )?);
        }
        let overlay = schema::overlay_of(table);
        let pk = spec::pk_name(&table.as_type(), Some(&overlay));
        let trigger = if schema::has_audit_columns(table) {
            sql_dialect::render_sql(
                dialect,
                &table_statements::create_trigger(dialect, table, mapping, &pk)?,
            )?
        } else {
            String::new()
        };
        body.push_str(&sql_dialect::render_sql(
            dialect,
            &table_statements::create_table(dialect, table, mapping)?,
        )?);
        body.push('\n');
        let nonempty: Vec<_> = indexes.into_iter().filter(|sql| !sql.is_empty()).collect();
        if !nonempty.is_empty() {
            body.push_str(&nonempty.join("\n"));
            body.push('\n');
        }
        if !trigger.is_empty() {
            body.push_str(&trigger);
            body.push('\n');
        }
        body.push('\n');
    }
    if !seeds.is_empty() {
        body.push_str("-- Seed data\n");
        for block in seeds {
            body.push_str(block);
            if !block.ends_with('\n') {
                body.push('\n');
            }
        }
    }
    Ok(body)
}

fn down_sql(dialect: &str, live: &[SqlTable]) -> Result<String> {
    let mut body = format!("-- Rollback initial migration for {dialect}\n\n-- DROP tables (reverse dependency order)\n");
    for table in live.iter().rev() {
        body.push_str(&sql_dialect::render_sql(
            dialect,
            &table_statements::drop_table(dialect, &table.table_name)?,
        )?);
        body.push('\n');
    }
    Ok(body)
}

fn finalize(text: &str, casing: &PackCasing) -> String {
    let collapsed = Regex::new(r"\n{3,}").expect("nl").replace_all(text, "\n\n");
    let output = casing.apply_keywords(&collapsed);
    if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    }
}
