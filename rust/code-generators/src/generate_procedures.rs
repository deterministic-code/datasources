use crate::by_groups;
use crate::casing::PackCasing;
use crate::generate_entry::GenerateEntry;
use crate::procedure_fields;
use crate::schema::{self, SqlTable};
use crate::spec;
use crate::sql_dialect;
use crate::write_inclusion;
use crate::table_statements;
use datasources::{
    procedure_name_of, ProcedureByGroup, ProcedureSource, Result, RoutineParameter, Statement,
};
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
    let tables: Vec<_> = schema::build_live(&types, &casing.strategy)
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
        let statements = for_table(dialect, t, &casing, &groups)?;
        let mut created = Vec::new();
        for statement in &statements {
            created.push(sql_dialect::render_sql(dialect, statement.as_ref())?);
            drops.push(sql_dialect::render_sql(
                dialect,
                &table_statements::drop_routine(&procedure_name_of(statement.as_ref())),
            )?);
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
    casing: &PackCasing,
    groups: &[Vec<String>],
) -> Result<Vec<Box<dyn Statement>>> {
    let pk = procedure_fields::pk_of(t);
    let param = |field: &crate::spec::SpecTypeField| {
        RoutineParameter::new(
            datasources::CreateTableStatement::column_name(&schema::field_stem(t, &field.name), &casing.strategy),
            field.type_name.clone(),
        )
        .with_field(field.converter_field())
    };
    let column = |logical: &str| {
        datasources::CreateTableStatement::column_name(&schema::field_stem(t, logical), &casing.strategy)
    };
    let by_groups = groups
        .iter()
        .map(|logical| {
            let parameters = logical
                .iter()
                .map(|name| {
                    let field = t.fields.iter().find(|f| f.name == *name).cloned().unwrap_or_else(|| pk.clone());
                    param(&field)
                })
                .collect::<Vec<_>>();
            let mut update = parameters.clone();
            for f in procedure_fields::update_params(t) {
                if !logical.iter().any(|name| name == &f.name) {
                    update.push(param(&f));
                }
            }
            ProcedureByGroup {
                logical: logical.clone(),
                columns: logical.iter().map(|name| column(name)).collect(),
                parameters,
                update_parameters: update,
            }
        })
        .collect();
    let type_ = t.as_type();
    let overlay = schema::overlay_of(t);
    let source = ProcedureSource {
        plural: casing.plural_stem(&t.name),
        table: t.table_name.clone(),
        columns: t.fields.iter().map(|f| column(&f.name)).collect(),
        create_parameters: procedure_fields::create_params(t).into_iter().map(|f| param(&f)).collect(),
        writable: procedure_fields::update_params(t).iter().map(|f| column(&f.name)).collect(),
        create_columns: procedure_fields::create_params(t).into_iter().map(|f| param(&f)).collect(),
        update_columns: {
            let mut cols = vec![param(&pk)];
            for f in procedure_fields::update_params(t) {
                if f.name != pk.name {
                    cols.push(param(&f));
                }
            }
            cols
        },
        typed_columns: t.fields.iter().map(param).collect(),
        by_groups,
        key: column(&pk.name),
        key_type: pk.type_name.clone(),
        audit_updated: t
            .fields
            .iter()
            .find(|f| f.name == "updated")
            .filter(|f| write_inclusion::include_in_update(f, &type_, Some(&overlay)))
            .map(|_| column("updated")),
        generated_key: pk.type_name == "uuid",
        routine_name: Box::new({
            let casing = casing.clone();
            move |stem: &str| casing.routine_name(stem)
        }),
    };
    Ok(sql_dialect::driver_of(dialect)?
        .statement_renderer()
        .expect("sql")
        .procedures(&source))
}

fn with_nl(text: &str, casing: &PackCasing, dialect: &str) -> Result<String> {
    let output = sql_dialect::schema_renderer(dialect, casing.strategy.clone())?.apply_keywords(text);
    Ok(if output.ends_with('\n') {
        output
    } else {
        format!("{output}\n")
    })
}
