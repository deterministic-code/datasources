use crate::mapping::SqlMapping;
use crate::schema::{self, SqlTable};
use crate::spec::{self, SeedRow, SpecTypeField};
use crate::sql_dialect;
use crate::table_statements;
use datasources::{DefaultValueConverter, Result};
use serde_yaml::Value;
use sha1::{Digest, Sha1};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

const NAMESPACE: &str = "9b3a8e6c-2f1d-4a5b-8c9d-1e2f3a4b5c6d";

pub fn sections(
    dialect: &str,
    tables: &[SqlTable],
    seeds: &HashMap<String, Vec<SeedRow>>,
    mapping: &SqlMapping,
) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for table in tables {
        let Some(rows) = seeds.get(&table.name).filter(|rows| !rows.is_empty()) else {
            continue;
        };
        let generated = if schema::is_generated_identity(table) {
            schema::table_identity(table)
        } else {
            Vec::new()
        };
        let pk = table
            .fields
            .iter()
            .find(|field| schema::table_identity(table).first().map(String::as_str) == Some(field.name.as_str()));
        let identity = match pk {
            Some(field) => {
                schema::uses_generated_id_column(table, field)
                    && sql_dialect::sql_default(dialect, &field.converter_field())?.is_none()
            }
            None => false,
        };
        let mut inserts = Vec::new();
        for seed in rows {
            let cols = cols_for_row(table, &seed.row, &generated);
            let mut values = Vec::new();
            for c in &cols {
                values.push(if generated.iter().any(|g| g == c) {
                    id_value(table, seed.id)
                } else if c == "uuid" {
                    DefaultValueConverter::sql_string_literal(seed_uuid(&table.name, seed.id))
                } else {
                    col_value(dialect, table.fields.iter().find(|f| f.name == *c), seed.row.get(c))?
                });
            }
            inserts.push(table_statements::seed_insert(table, mapping, &cols, &values, dialect)?);
        }
        lines.push(format!("-- Seeds: {}", table.table_name));
        for insert in inserts {
            lines.push(sql_dialect::render_sql(
                dialect,
                &table_statements::identity_insert(table, mapping, vec![insert], identity),
            )?);
        }
    }
    Ok(lines)
}

pub fn added(
    previous: &HashMap<String, Vec<SeedRow>>,
    current: &HashMap<String, Vec<SeedRow>>,
) -> HashMap<String, Vec<SeedRow>> {
    let mut added = HashMap::new();
    for (table, rows) in current {
        let ids: HashSet<i32> = previous
            .get(table)
            .map(|prior| prior.iter().map(|row| row.id).collect())
            .unwrap_or_default();
        let next: Vec<SeedRow> = rows.iter().filter(|row| !ids.contains(&row.id)).cloned().collect();
        if !next.is_empty() {
            added.insert(table.clone(), next);
        }
    }
    added
}

pub fn deletes(
    dialect: &str,
    tables: &[SqlTable],
    seeds: &HashMap<String, Vec<SeedRow>>,
    mapping: &SqlMapping,
) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for table in tables {
        let Some(rows) = seeds.get(&table.name).filter(|rows| !rows.is_empty()) else {
            continue;
        };
        let id = schema::table_identity(table)
            .into_iter()
            .next()
            .unwrap_or_else(|| "id".into());
        lines.push(format!("-- Seeds: {}", table.table_name));
        for seed in rows {
            lines.push(format!(
                "DELETE FROM {} WHERE {} = {};",
                sql_dialect::q(dialect, &table.table_name)?,
                sql_dialect::q(dialect, &mapping.column_name(table, &id))?,
                id_value(table, seed.id)
            ));
        }
    }
    Ok(lines)
}

fn pk_type(table: &SqlTable) -> String {
    let overlay = schema::overlay_of(table);
    table
        .fields
        .iter()
        .find(|f| spec::is_pk_field(f, &table.as_type(), Some(&overlay)))
        .map(|f| f.type_name.clone())
        .unwrap_or_else(|| "integer".into())
}

fn seed_uuid(table_name: &str, id: i32) -> String {
    let ns = hex_bytes(NAMESPACE);
    let mut hasher = Sha1::new();
    hasher.update(ns);
    hasher.update(format!("{table_name}:{id}").as_bytes());
    let hash = hasher.finalize();
    let mut bytes = hash[..16].to_vec();
    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

fn hex_bytes(uuid: &str) -> Vec<u8> {
    let hex = uuid.replace('-', "");
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap_or(0))
        .collect()
}

fn truthy(value: &Value) -> bool {
    match value {
        Value::Bool(true) => true,
        Value::Number(n) => n.as_i64() == Some(1),
        Value::String(text) => text == "true" || text == "True",
        _ => false,
    }
}

fn render_value(dialect: &str, field: &SpecTypeField, value: &Value) -> Result<String> {
    if field.type_name == "boolean" {
        return Ok(sql_dialect::driver_of(dialect)?
            .default_value_converter
            .convert("boolean", if truthy(value) { "true" } else { "false" })?
            .unwrap_or_else(|| "0".into()));
    }
    if field.type_name == "integer" || field.type_name == "reference" {
        return Ok(spec::yaml_text(value));
    }
    Ok(DefaultValueConverter::sql_string_literal(spec::yaml_text(value)))
}

fn id_value(table: &SqlTable, id: i32) -> String {
    match pk_type(table).as_str() {
        "uuid" => DefaultValueConverter::sql_string_literal(seed_uuid(&table.name, id)),
        "string" => DefaultValueConverter::sql_string_literal(id.to_string()),
        _ => id.to_string(),
    }
}

fn col_value(dialect: &str, field: Option<&SpecTypeField>, value: Option<&Value>) -> Result<String> {
    let Some(field) = field else {
        return Ok("NULL".into());
    };
    match value {
        None | Some(Value::Null) => Ok(sql_dialect::sql_default(dialect, &field.converter_field())?
            .unwrap_or_else(|| "NULL".into())),
        Some(cell) => render_value(dialect, field, cell),
    }
}

fn cols_for_row(table: &SqlTable, row: &IndexMap<String, Value>, generated: &[String]) -> Vec<String> {
    let names: HashSet<&str> = table.fields.iter().map(|f| f.name.as_str()).collect();
    let mut cols = generated
        .iter()
        .filter(|c| names.contains(c.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if names.contains("uuid") && !generated.iter().any(|g| g == "uuid") {
        cols.push("uuid".into());
    }
    cols.extend(
        row.keys()
            .filter(|k| !generated.iter().any(|g| g == *k) && *k != "uuid")
            .cloned(),
    );
    cols
}
