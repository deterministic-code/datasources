use crate::by_fields;
use crate::casing::PackCasing;
use crate::generate_entry::GenerateEntry;
use crate::mapping::SqlMapping;
use crate::procedure_fields;
use crate::procedure_specs;
use crate::schema;
use crate::spec;
use crate::sql_dialect;
use crate::table_statements;
use datasources::{DatasourceError, Result, RoutineKind, RoutineParameter};
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
    let by_fields = by_fields::from(&types);
    let mut bodies = Vec::new();
    let mut drops = Vec::new();
    for t in &tables {
        let fields = by_fields.get(&t.name).cloned().unwrap_or_default();
        let pk = procedure_fields::pk_of(t);
        let columns: Vec<String> = t.fields.iter().map(|f| mapping.column_name(t, &f.name)).collect();
        let writable: Vec<String> = procedure_fields::writable_non_audit(t)
            .iter()
            .map(|f| mapping.column_name(t, &f.name))
            .collect();
        let mut created = Vec::new();
        for proc in procedure_specs::of(&t.name, &fields, &casing) {
            let key_field = proc
                .by_field
                .as_ref()
                .and_then(|by| t.fields.iter().find(|f| f.name == *by).cloned())
                .unwrap_or_else(|| pk.clone());
            let mut params_list = vec![RoutineParameter {
                name: mapping.column_name(t, &key_field.name),
                r#type: sql_dialect::map_column_type(dialect, &key_field.converter_field())?,
            }];
            for f in procedure_fields::writable_non_audit(t) {
                if Some(&f.name) != proc.by_field.as_ref() {
                    params_list.push(RoutineParameter {
                        name: mapping.column_name(t, &f.name),
                        r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
                    });
                }
            }
            if t.fields.iter().any(|f| f.name == "updated") {
                let updated = t.fields.iter().find(|f| f.name == "updated").expect("updated");
                params_list.push(RoutineParameter {
                    name: mapping.column_name(t, "new_updated"),
                    r#type: sql_dialect::map_column_type(dialect, &updated.converter_field())?,
                });
            }
            let create_params = procedure_fields::create_params(t)
                .into_iter()
                .map(|f| {
                    Ok(RoutineParameter {
                        name: mapping.column_name(t, &f.name),
                        r#type: sql_dialect::map_column_type(dialect, &f.converter_field())?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let kind = if proc.op == "updateBy" { "update" } else { proc.op.as_str() };
            let parameters = if proc.op == "create" {
                create_params
            } else if proc.op.starts_with("update") {
                params_list
            } else if proc.op == "findBy" {
                vec![RoutineParameter {
                    name: mapping.column_name(t, &key_field.name),
                    r#type: sql_dialect::map_column_type(dialect, &key_field.converter_field())?,
                }]
            } else {
                Vec::new()
            };
            let sql = sql_dialect::render_sql(
                dialect,
                &table_statements::create_routine(
                    &proc.name,
                    kind_of(kind)?,
                    sql_dialect::q(dialect, &t.table_name)?,
                    columns.clone(),
                    parameters,
                    mapping.column_name(t, &pk.name),
                    sql_dialect::map_column_type(dialect, &pk.converter_field())?,
                    writable.clone(),
                    proc.by_field.as_ref().map(|by| mapping.column_name(t, by)),
                    pk.type_name == "uuid",
                    if t.fields.iter().any(|f| f.name == "updated") {
                        Some(mapping.column_name(t, "updated"))
                    } else {
                        None
                    },
                ),
            )?;
            if !sql.is_empty() {
                created.push(sql);
            }
            let drop = sql_dialect::render_sql(dialect, &table_statements::drop_routine(&proc.name))?;
            if !drop.is_empty() {
                drops.push(drop);
            }
        }
        if !created.is_empty() {
            bodies.push(format!("-- {}\n\n{}", t.name, created.join("\n\n")));
        }
    }
    if bodies.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![
        GenerateEntry::new(
            format!("{dialect}/migrations/{order:04}_stored_procedures_up.sql"),
            with_nl(&format!("-- Stored procedures\n{}", bodies.join("\n\n")), &casing),
        ),
        GenerateEntry::new(
            format!("{dialect}/migrations/{order:04}_stored_procedures_down.sql"),
            with_nl(&format!("-- DROP stored procedures\n{}", drops.join("\n")), &casing),
        ),
    ])
}

fn kind_of(op: &str) -> Result<RoutineKind> {
    Ok(match op {
        "create" => RoutineKind::Create,
        "findOne" => RoutineKind::FindOne,
        "findAll" => RoutineKind::FindAll,
        "findBy" => RoutineKind::FindBy,
        "update" => RoutineKind::Update,
        "delete" => RoutineKind::Delete,
        _ => return Err(DatasourceError::message(format!("{op}: empty step"))),
    })
}

fn with_nl(text: &str, casing: &PackCasing) -> String {
    let output = casing.apply_keywords(text);
    if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    }
}
