use crate::by_groups;
use crate::casing::PackCasing;
use crate::generate_entry::GenerateEntry;
use crate::mapping::SqlMapping;
use crate::procedure_fields;
use crate::procedure_spec::ProcedureSpec;
use crate::schema::{self, SqlTable};
use crate::spec;
use crate::sql_dialect;
use crate::write_inclusion;
use crate::table_statements;
use datasources::{DatasourceError, Result, RoutineParameter, Statement};
use std::collections::HashMap;
use std::sync::Arc;

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
                dialect,
            )?,
        ),
        GenerateEntry::new(
            format!("{dialect}/migrations/{order:04}_stored_procedures_down.sql"),
            with_nl(
                &format!(
                    "-- DROP stored procedures\n{}",
                    crate::join_batches::join(&drops, &separator)
                ),
                &casing,
                dialect,
            )?,
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
    let param = |field: &crate::spec::SpecTypeField| {
        RoutineParameter::new(mapping.column_name(t, &field.name), field.type_name.clone())
            .with_field(field.converter_field())
    };
    let columns: Vec<String> = t.fields.iter().map(|f| mapping.column_name(t, &f.name)).collect();
    let writable: Vec<String> = procedure_fields::update_params(t)
        .iter()
        .map(|f| mapping.column_name(t, &f.name))
        .collect();
    let created_params = procedure_fields::create_params(t).into_iter().map(|f| param(&f)).collect::<Vec<_>>();
    let typed_columns = t.fields.iter().map(param).collect::<Vec<_>>();
    let payload = RoutineParameter::new("payload", "payload");
    let key = mapping.column_name(t, &pk.name);
    let key_type = pk.type_name.clone();
    let create_columns = procedure_fields::create_params(t).into_iter().map(|f| param(&f)).collect::<Vec<_>>();
    let mut update_columns = vec![param(&pk)];
    for f in procedure_fields::update_params(t) {
        if f.name != pk.name {
            update_columns.push(param(&f));
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
    let table = t.table_name.clone();
    let mut statements: Vec<Box<dyn Statement>> = Vec::new();
    let mut names = Vec::new();
    for proc in specs_of(dialect, &t.name, groups, casing)? {
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
                param(&field)
            })
            .collect::<Vec<_>>();
        let mut update_params = by_params.clone();
        for f in procedure_fields::update_params(t) {
            if !proc.by_properties.iter().any(|name| name == &f.name) {
                update_params.push(param(&f));
            }
        }
        let boxed: Box<dyn Statement> = match proc.op.as_str() {
            "create" => Box::new(table_statements::create_create_stored_procedure(
                &proc.name, &table, columns.clone(), created_params.clone(), &key, &key_type, writable.clone(), generated_key, audit.clone(),
            )),
            "read" => Box::new(table_statements::create_read_stored_procedure(
                &proc.name, &table, columns.clone(), by_params.clone(), &key, &key_type, writable.clone(), by.clone(), generated_key, audit.clone(),
            )),
            "update" => Box::new(table_statements::create_update_stored_procedure(
                &proc.name, &table, columns.clone(), update_params, &key, &key_type, writable.clone(), by, generated_key, audit.clone(),
            )),
            "delete" => Box::new(table_statements::create_delete_stored_procedure(
                &proc.name, &table, columns.clone(), by_params.clone(), &key, &key_type, writable.clone(), by, generated_key, audit.clone(),
            )),
            "bulkCreate" => Box::new(table_statements::create_bulk_create_stored_procedure(
                &proc.name, &table, create_columns.clone(), payload.clone(), &key, &key_type,
            )),
            "bulkUpdate" => Box::new(table_statements::create_bulk_update_stored_procedure(
                &proc.name, &table, update_columns.clone(), payload.clone(), &key, &key_type,
            )),
            "bulkDelete" => Box::new(table_statements::create_bulk_delete_stored_procedure(
                &proc.name, &table, typed_columns.clone(), payload.clone(), &key, &key_type,
            )),
            _ => return Err(DatasourceError::message(format!("{}: empty step", proc.op))),
        };
        statements.push(boxed);
    }
    Ok((statements, names))
}

fn specs_of(
    dialect: &str,
    entity: &str,
    groups: &[Vec<String>],
    casing: &PackCasing,
) -> Result<Vec<ProcedureSpec>> {
    let _driver: Arc<dyn datasources::Driver> = sql_dialect::driver_of(dialect)?;
    let rows = match dialect {
        "sqlite" | "mysql" | "postgres" | "sqlserver" | "oracle" => procedure_specs(
            entity,
            groups,
            &|stem| casing.routine_name(stem),
            &|name| casing.plural_stem(name),
        ),
        _ => return Err(DatasourceError::message(format!("\"{dialect}\" has no ProcedureSpecs."))),
    };
    Ok(rows
        .into_iter()
        .map(|(op, name, by)| ProcedureSpec::new(op, name, by))
        .collect())
}

fn procedure_specs(
    entity_name: &str,
    by_groups: &[Vec<String>],
    routine_name: &dyn Fn(&str) -> String,
    plural_stem: &dyn Fn(&str) -> String,
) -> Vec<(String, String, Vec<String>)> {
    let plural = plural_stem(entity_name);
    let suffix = |props: &[String]| {
        if props.is_empty() {
            String::new()
        } else {
            format!("_by_{}", props.join("_"))
        }
    };
    let mut out = vec![
        ("create".into(), routine_name(&format!("create_{plural}")), vec![]),
        ("read".into(), routine_name(&format!("read_{plural}")), vec![]),
    ];
    for props in by_groups {
        let by = suffix(props);
        out.push(("read".into(), routine_name(&format!("read_{plural}{by}")), props.clone()));
        out.push(("update".into(), routine_name(&format!("update_{plural}{by}")), props.clone()));
        out.push(("delete".into(), routine_name(&format!("delete_{plural}{by}")), props.clone()));
    }
    out.push(("bulkCreate".into(), routine_name(&format!("bulk_create_{plural}")), vec![]));
    out.push(("bulkUpdate".into(), routine_name(&format!("bulk_update_{plural}")), vec![]));
    out.push(("bulkDelete".into(), routine_name(&format!("bulk_delete_{plural}")), vec![]));
    out
}

fn with_nl(text: &str, casing: &PackCasing, dialect: &str) -> Result<String> {
    let output = casing.apply_keywords(text, dialect)?;
    Ok(if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    })
}
