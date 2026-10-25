use crate::casing::PackCasing;
use crate::by_groups;
use crate::generate_entry::GenerateEntry;
use crate::generate_procedures;
use crate::mapping::SqlMapping;
use crate::schema::{self, SqlTable};
use crate::schema_diff::{self, DdlStatement};
use crate::seed_generator;
use crate::spec::{self, SeedRow};
use crate::sql_dialect;
use datasources::{DatasourceError, Result};
use deterministic_generators_common::{VersionSnapshot, YamlNode, YamlValue};
use indexmap::IndexMap;
use regex::Regex;
use std::collections::HashMap;

const DEFAULT_DATASOURCE: &str = "version: 1.0.0\nincludes:\n  - types:\n      filter: tag == \"datasource_type\" || tag == \"view_type\"\ntypes: []\n";

pub fn diff_paths(files: &HashMap<String, String>) -> Vec<String> {
    let pattern = Regex::new(r"^version-\d{8}T\d{6}Z-\d{3}/diff\.yaml$").expect("version");
    let mut paths: Vec<String> = files.keys().filter(|path| pattern.is_match(path)).cloned().collect();
    paths.sort();
    paths
}

pub fn files(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
    casing: &PackCasing,
) -> Result<Vec<GenerateEntry>> {
    let paths = diff_paths(files);
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let mut parsed = Vec::new();
    let mut snapshots = Vec::new();
    for path in &paths {
        let text = files.get(path).map(String::as_str).unwrap_or("");
        if text.is_empty() {
            continue;
        }
        parsed.push(parse_diff(text, path)?);
        snapshots.push(combine(parsed.iter().cloned().map(Some))?);
    }
    if snapshots.is_empty() {
        return Ok(Vec::new());
    }
    let last = snapshots.last().unwrap();
    if !same(last, "types.yaml", files.get("types.yaml").map(String::as_str), None)?
        || !same(last, "datasource.yaml", files.get("datasource.yaml").map(String::as_str), Some(DEFAULT_DATASOURCE))?
    {
        return Err(DatasourceError::message(
            "types.yaml or datasource.yaml differs from the frozen version diffs; run datasources version first",
        ));
    }

    let mut output = Vec::new();
    let mut previous = compile(&combine(std::iter::empty::<Option<YamlValue>>())?, settings)?;
    let mut number = 0;
    for (index, snapshot) in snapshots.iter().enumerate() {
        let current = compile(snapshot, settings)?;
        let up = schema_diff::of(dialect, &previous.0, &previous.1, &current.0, &current.1)?;
        let down = schema_diff::of(dialect, &current.0, &current.1, &previous.0, &previous.1)?;
        let up_procs = procedure_sql(dialect, settings, &previous.0, &current.0, &current.1, casing)?;
        let down_procs = procedure_sql(dialect, settings, &current.0, &previous.0, &previous.1, casing)?;
        if up.is_empty() && down.is_empty() && up_procs.is_empty() && down_procs.is_empty() {
            previous = current;
            continue;
        }
        number += 1;
        let seeds = if index == 0 && up.iter().any(DdlStatement::is_create_table) {
            seed_generator::sections(dialect, &current.0, &current.2, &current.1)?
        } else {
            Vec::new()
        };
        let mut up_sql = format!("-- Generated migration for {dialect}\n{}", render(dialect, &up)?);
        if !up_procs.is_empty() {
            if !up_sql.ends_with('\n') {
                up_sql.push('\n');
            }
            up_sql.push_str(&up_procs);
        }
        if !seeds.is_empty() {
            up_sql.push_str("\n\n-- Seed data\n");
            for block in seeds {
                up_sql.push_str(&block);
                if !block.ends_with('\n') {
                    up_sql.push('\n');
                }
            }
        }
        let mut down_sql = format!("-- Rollback migration for {dialect}\n{}", render(dialect, &down)?);
        if !down_procs.is_empty() {
            if !down_sql.ends_with('\n') {
                down_sql.push('\n');
            }
            down_sql.push_str(&down_procs);
        }
        output.push(GenerateEntry::new(
            format!("{:04}_up.sql", number),
            finalize(&up_sql, casing),
        ));
        output.push(GenerateEntry::new(
            format!("{:04}_down.sql", number),
            finalize(&down_sql, casing),
        ));
        previous = current;
    }
    Ok(output)
}

fn combine(diffs: impl IntoIterator<Item = Option<YamlValue>>) -> Result<VersionSnapshot> {
    VersionSnapshot::combine(diffs).map_err(|error| DatasourceError::message(error.to_string()))
}

fn parse_diff(text: &str, source: &str) -> Result<YamlValue> {
    let node = YamlNode::from_yaml(text).map_err(|error| DatasourceError::message(format!("{source}: {error}")))?;
    match node.value().cloned() {
        Some(value @ YamlValue::Map(_)) => Ok(value),
        _ => Err(DatasourceError::message(format!("{source}: expected a mapping"))),
    }
}

fn fold(path: &str, text: &str) -> Result<String> {
    let content = YamlNode::from_yaml(text)
        .map_err(|error| DatasourceError::message(format!("{path}: {error}")))?
        .value()
        .cloned()
        .unwrap_or(YamlValue::Null);
    let mut envelope = IndexMap::new();
    envelope.insert("added".into(), YamlValue::Bool(true));
    envelope.insert("content".into(), content);
    let mut doc = IndexMap::new();
    doc.insert(path.to_string(), YamlValue::Map(envelope));
    combine([Some(YamlValue::Map(doc))])?.yaml(path).map_err(|error| DatasourceError::message(error.to_string()))
}

fn same(snapshot: &VersionSnapshot, path: &str, live: Option<&str>, missing: Option<&str>) -> Result<bool> {
    let has = snapshot.documents.contains_key(path);
    let live_text = live.filter(|text| !text.is_empty()).or(missing);
    let Some(live_text) = live_text else {
        return Ok(!has);
    };
    let live_yaml = fold(path, live_text)?;
    let snapshot_yaml = if has {
        snapshot.yaml(path).map_err(|error| DatasourceError::message(error.to_string()))?
    } else {
        match missing {
            Some(text) => fold(path, text)?,
            None => return Ok(false),
        }
    };
    Ok(snapshot_yaml == live_yaml)
}

fn compile(
    snapshot: &VersionSnapshot,
    settings: &HashMap<String, String>,
) -> Result<(Vec<SqlTable>, SqlMapping, HashMap<String, Vec<SeedRow>>)> {
    let mut files = HashMap::new();
    for path in snapshot.documents.keys() {
        files.insert(
            path.clone(),
            snapshot.yaml(path).map_err(|error| DatasourceError::message(error.to_string()))?,
        );
    }
    files.entry("datasource.yaml".into()).or_insert_with(|| DEFAULT_DATASOURCE.into());
    let parsed = spec::parse(&files)?;
    let types = schema::from_spec(&parsed);
    let mapping = SqlMapping::new(PackCasing::of(settings)?, &types);
    Ok((schema::build_live(&types, &mapping), mapping, parsed.datasource_seeds))
}

fn fields_of(table: &SqlTable) -> String {
    table
        .fields
        .iter()
        .map(|field| format!("{}:{}", field.name, field.type_name))
        .collect::<Vec<_>>()
        .join("|")
}

fn procedure_sql(
    dialect: &str,
    settings: &HashMap<String, String>,
    before: &[SqlTable],
    after: &[SqlTable],
    after_map: &SqlMapping,
    casing: &PackCasing,
) -> Result<String> {
    if !schema::use_stored_procedures(settings) {
        return Ok(String::new());
    }
    let previous: HashMap<_, _> = before.iter().map(|table| (table.name.as_str(), table)).collect();
    let groups = by_groups::from(after);
    let mut parts = Vec::new();
    for table in after {
        let Some(prior) = previous.get(table.name.as_str()) else { continue };
        if !schema::has_audit_columns(table) || fields_of(prior) == fields_of(table) {
            continue;
        }
        let group = groups.get(&table.name).cloned().unwrap_or_default();
        let (statements, _) = generate_procedures::for_table(dialect, table, after_map, casing, &group)?;
        for statement in statements {
            let sql = sql_dialect::render_sql(dialect, statement.as_ref())?;
            if !sql.is_empty() {
                parts.push(sql);
            }
        }
    }
    Ok(parts.join("\n"))
}

fn render(dialect: &str, statements: &[DdlStatement]) -> Result<String> {
    let mut parts = Vec::new();
    for statement in statements {
        let sql = sql_dialect::render_sql(dialect, statement.as_statement())?;
        if !sql.is_empty() {
            parts.push(sql);
        }
    }
    Ok(parts.join("\n"))
}

fn finalize(text: &str, casing: &PackCasing) -> String {
    let collapsed = regex::Regex::new(r"\n{3,}").expect("nl").replace_all(text, "\n\n");
    let output = casing.apply_keywords(&collapsed);
    if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    }
}
