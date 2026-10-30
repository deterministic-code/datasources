use crate::schema::{self, field_stem, SqlTable};
use datasources::{CreateTableStatement, StatementNaming};
use crate::spec::{self, SeedRow, SpecTypeField};
use crate::sql_dialect;
use datasources::{Result, SpecField, SqlSeedColumn, SqlSeedStatement};
use serde_yaml::Value;
use sha1::{Digest, Sha1};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

const NAMESPACE: &str = "9b3a8e6c-2f1d-4a5b-8c9d-1e2f3a4b5c6d";

pub fn sections(
    dialect: &str,
    tables: &[SqlTable],
    seeds: &HashMap<String, Vec<SeedRow>>,
    naming: &StatementNaming,
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
            Some(field) => schema::uses_generated_id_column(table, field) && !field.has_default,
            None => false,
        };
        lines.push(format!("-- Seeds: {}", table.table_name));
        for seed in rows {
            let cols = cols_for_row(table, &seed.row, &generated);
            let columns = cols
                .iter()
                .map(|c| column_of(table, naming, c, seed))
                .collect();
            lines.push(sql_dialect::render_sql(
                dialect,
                &SqlSeedStatement::new(
                    table.table_name.clone(),
                    columns,
                    "insert",
                    identity,
                    pk.map(|field| CreateTableStatement::column_name(&field_stem(table, &field.name), naming)),
                ),
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
    naming: &StatementNaming,
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
        let field = table.fields.iter().find(|f| f.name == id);
        lines.push(format!("-- Seeds: {}", table.table_name));
        for seed in rows {
            let spec = field
                .map(SpecTypeField::converter_field)
                .unwrap_or(SpecField {
                    type_name: pk_type(table),
                    ..SpecField::default()
                });
            lines.push(sql_dialect::render_sql(
                dialect,
                &SqlSeedStatement::new(
                    table.table_name.clone(),
                    vec![SqlSeedColumn {
                        column: CreateTableStatement::column_name(&field_stem(table, &id), naming),
                        field: spec,
                        value: Some(id_value(table, seed.id)),
                    }],
                    "delete",
                    false,
                    None,
                ),
            )?);
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

fn id_value(table: &SqlTable, id: i32) -> String {
    match pk_type(table).as_str() {
        "uuid" => seed_uuid(&table.name, id),
        "string" => id.to_string(),
        _ => id.to_string(),
    }
}

fn column_of(table: &SqlTable, naming: &StatementNaming, logical: &str, seed: &SeedRow) -> SqlSeedColumn {
    let field = table.fields.iter().find(|f| f.name == logical);
    let spec = field.map(SpecTypeField::converter_field).unwrap_or(SpecField {
        type_name: "string".into(),
        name: Some(logical.into()),
        ..SpecField::default()
    });
    let generated = if schema::is_generated_identity(table) {
        schema::table_identity(table)
    } else {
        Vec::new()
    };
    let value = if generated.iter().any(|g| g == logical) {
        Some(id_value(table, seed.id))
    } else if logical == "uuid" {
        Some(seed_uuid(&table.name, seed.id))
    } else {
        seed.row.get(logical).filter(|v| !matches!(v, Value::Null)).map(spec::yaml_text)
    };
    SqlSeedColumn {
        column: CreateTableStatement::column_name(&field_stem(table, logical), naming),
        field: spec,
        value,
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
