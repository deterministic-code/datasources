use crate::casing::PackCasing;
use crate::procedure_spec::ProcedureSpec;

pub fn of(entity_name: &str, by_fields: &[String], casing: &PackCasing) -> Vec<ProcedureSpec> {
    let mut specs = vec![
        ProcedureSpec::new("create", casing.routine_name(&format!("create_{entity_name}")), None),
        ProcedureSpec::new("findOne", casing.routine_name(&format!("find_{entity_name}")), None),
        ProcedureSpec::new(
            "findAll",
            casing.routine_name(&format!("find_{}", casing.plural_table_name(entity_name))),
            None,
        ),
    ];
    for bf in by_fields {
        specs.push(ProcedureSpec::new(
            "findBy",
            casing.routine_name(&format!("find_{entity_name}_by_{bf}")),
            Some(bf.clone()),
        ));
    }
    specs.push(ProcedureSpec::new(
        "update",
        casing.routine_name(&format!("update_{entity_name}")),
        None,
    ));
    for bf in by_fields {
        specs.push(ProcedureSpec::new(
            "updateBy",
            casing.routine_name(&format!("update_{entity_name}_by_{bf}")),
            Some(bf.clone()),
        ));
    }
    specs.push(ProcedureSpec::new(
        "delete",
        casing.routine_name(&format!("delete_{entity_name}")),
        None,
    ));
    specs
}
