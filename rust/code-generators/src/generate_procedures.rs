use crate::by_groups;
use crate::casing::PackCasing;
use crate::generate_entry::GenerateEntry;
use crate::mapping::SqlMapping;
use crate::procedure_fields;
use crate::procedure_specs;
use crate::schema::{self, SqlTable};
use crate::spec;
use crate::sql_dialect;
use crate::write_inclusion;
use crate::table_statements;
use datasources::{DatasourceError, Result, RoutineParameter, Statement};
use std::collections::HashMap;

pub fn for_dialect(
    dialect: &str,
    files: &HashMap<String, String>,
    settings: &HashMap<String, String>,
    order: i32,
) -> Result<Vec<GenerateEntry>> {
    if !schema::use_stored_procedures(settings) {
        return Ok(Vec::new());
    }
    let parsed = spec::parse(files)?;
    let types = schema::from_spec(&parsed);
    let casing = PackCasing::of(settings)?;
    let mapping = SqlMapping::new(casing.clone(), &types);
    let tables: Vec<_> = schema::build_live(&types, &mapping)
        .into_iter()
        .filter(schema::has_audit_columns)
        .collect();
    if tables.is_empty() {
        return Ok(Vec::new());
    }
    let by_groups = by_groups::from(&types);
    let separator = crate::join_batches::of(settings)?;
    let mut bodies = Vec::new();
    let mut drops = Vec::new();
    for t in &tables {
        let groups = by_groups.get(&t.name).cloned().unwrap_or_default();
        let (statements, names) = for_table(dialect, t, &mapping, &casing, &groups)?;
        let mut created = Vec::new();
        for statement in statements {
            created.push(sql_dialect::render_sql(dialect, statement.as_ref())?);
        }
        for name in names {
            drops.push(sql_dialect::render_sql(dialect, &table_statements::drop_routine(&name))?);
        }
        if created.iter().any(|sql| !sql.is_empty()) {
            bodies.push(format!("-- {}\n\n{}", t.name, crate::join_batches::join(&created, &separator)));
        }
    }
    if bodies.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![
        GenerateEntry::new(
            format!("{dialect}/migrations/{order:04}_stored_procedures_up.sql"),
            with_nl(
                &format!("-- Stored procedures\n{}", crate::join_batches::join(&bodies, &separator)),
                &casing,
            ),
        ),
        GenerateEntry::new(
            format!("{dialect}/migrations/{order:04}_stored_procedures_down.sql"),
            with_nl(
                &format!(
                    "-- DROP stored procedures\n{}",
                    crate::join_batches::join(&drops, &separator)
                ),
                &casing,
            ),
        ),
    ])
}

pub fn for_table(
    dialect: &str,
    t: &SqlTable,
    mapping: &SqlMapping,
    casing: &PackCasing,
    groups: &[Vec<String>],
) -> Result<(Vec<Box<dyn Statement>>, Vec<String>)> {
    let pk = procedure_fields::pk_of(t);
    let columns: Vec<String> = t.fields.iter().map(|f| mapping.column_name(t, &f.name)).collect();
    let writable: Vec<String> = procedure_fields::update_params(t)
        .iter()
        .map(|f| mapping.column_name(t, &f.name))
        .collect();
    let created_params = procedure_fields::create_params(t)
        .into_iter()
        .map(|f| {
            Ok(RoutineParameter {
                name: mapping.column_name(t, &f.name),
                r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let typed_columns = t
        .fields
        .iter()
        .map(|f| {
            Ok(RoutineParameter {
                name: mapping.column_name(t, &f.name),
                r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let payload = RoutineParameter {
        name: "payload".into(),
        r#type: payload_type(dialect).into(),
    };
    let key = mapping.column_name(t, &pk.name);
    let key_type = sql_dialect::map_column_type(dialect, &pk.converter_field())?;
    let create_columns = procedure_fields::create_params(t)
        .into_iter()
        .map(|f| {
            Ok(RoutineParameter {
                name: mapping.column_name(t, &f.name),
                r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut update_columns = vec![RoutineParameter {
        name: key.clone(),
        r#type: key_type.clone(),
    }];
    for f in procedure_fields::update_params(t) {
        if f.name != pk.name {
            update_columns.push(RoutineParameter {
                name: mapping.column_name(t, &f.name),
                r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
            });
        }
    }
    let type_ = t.as_type();
    let overlay = schema::overlay_of(t);
    let audit = t
        .fields
        .iter()
        .find(|f| f.name == "updated")
        .filter(|f| write_inclusion::include_in_update(f, &type_, Some(&overlay)))
        .map(|_| mapping.column_name(t, "updated"));
    let generated_key = pk.type_name == "uuid";
    let quoted = sql_dialect::q(dialect, &t.table_name)?;
    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    let mut names = Vec::new();
    for proc in procedure_specs::of(&t.name, groups, casing) {
        names.push(proc.name.clone());
        let by: Vec<String> = proc.by_properties.iter().map(|logical| mapping.column_name(t, logical)).collect();
        let by_params = proc
            .by_properties
            .iter()
            .map(|logical| {
                let field = t
                    .fields
                    .iter()
                    .find(|f| f.name == *logical)
                    .cloned()
                    .unwrap_or_else(|| pk.clone());
                Ok(RoutineParameter {
                    name: mapping.column_name(t, &field.name),
                    r#type: sql_dialect::map_column_type(dialect, &field.converter_field())?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let mut update_params = by_params.clone();
        for f in procedure_fields::update_params(t) {
            if !proc.by_properties.iter().any(|name| name == &f.name) {
                update_params.push(RoutineParameter {
                    name: mapping.column_name(t, &f.name),
                    r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
                });
            }
        }
        let boxed: Box<dyn Statement> = match proc.op.as_str() {
            "create" => Box::new(table_statements::create_create_stored_procedure(
                &proc.name, &quoted, columns.clone(), created_params.clone(), &key, &key_type, writable.clone(), generated_key, audit.clone(),
            )),
            "read" => Box::new(table_statements::create_read_stored_procedure(
                &proc.name, &quoted, columns.clone(), by_params.clone(), &key, &key_type, writable.clone(), by.clone(), generated_key, audit.clone(),
            )),
            "update" => Box::new(table_statements::create_update_stored_procedure(
                &proc.name, &quoted, columns.clone(), update_params, &key, &key_type, writable.clone(), by, generated_key, audit.clone(),
            )),
            "delete" => Box::new(table_statements::create_delete_stored_procedure(
                &proc.name, &quoted, columns.clone(), by_params.clone(), &key, &key_type, writable.clone(), by, generated_key, audit.clone(),
            )),
            "bulkCreate" => Box::new(table_statements::create_bulk_create_stored_procedure(
                &proc.name, &quoted, create_columns.clone(), payload.clone(), &key, &key_type,
            )),
            "bulkUpdate" => Box::new(table_statements::create_bulk_update_stored_procedure(
                &proc.name, &quoted, update_columns.clone(), payload.clone(), &key, &key_type,
            )),
            "bulkDelete" => Box::new(table_statements::create_bulk_delete_stored_procedure(
                &proc.name, &quoted, typed_columns.clone(), payload.clone(), &key, &key_type,
            )),
            _ => return Err(DatasourceError::message(format!("{}: empty step", proc.op))),
        };
        statements.push(boxed);
    }
    Ok((statements, names))
}

fn payload_type(dialect: &str) -> &'static str {
    match dialect {
        "mysql" => "JSON",
        "postgres" => "jsonb",
        "sqlserver" => "NVARCHAR(MAX)",
        "oracle" => "CLOB",
        _ => "JSON",
    }
}

fn with_nl(text: &str, casing: &PackCasing) -> String {
    let output = casing.apply_keywords(text);
    if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    }
}
