use crate::casing::PackCasing;
use crate::procedure_spec::ProcedureSpec;

pub fn of(entity_name: &str, by_groups: &[Vec<String>], casing: &PackCasing) -> Vec<ProcedureSpec> {
    let plural = casing.plural_stem(entity_name);
    let suffix = |props: &[String]| {
        if props.is_empty() {
            String::new()
        } else {
            format!("_by_{}", props.join("_"))
        }
    };
    let mut specs = vec![
        ProcedureSpec::new("create", casing.routine_name(&format!("create_{plural}")), vec![]),
        ProcedureSpec::new("read", casing.routine_name(&format!("read_{plural}")), vec![]),
    ];
    for props in by_groups {
        let by = suffix(props);
        specs.push(ProcedureSpec::new(
            "read",
            casing.routine_name(&format!("read_{plural}{by}")),
            props.clone(),
        ));
        specs.push(ProcedureSpec::new(
            "update",
            casing.routine_name(&format!("update_{plural}{by}")),
            props.clone(),
        ));
        specs.push(ProcedureSpec::new(
            "delete",
            casing.routine_name(&format!("delete_{plural}{by}")),
            props.clone(),
        ));
    }
    specs.push(ProcedureSpec::new(
        "bulkCreate",
        casing.routine_name(&format!("bulk_create_{plural}")),
        vec![],
    ));
    specs.push(ProcedureSpec::new(
        "bulkUpdate",
        casing.routine_name(&format!("bulk_update_{plural}")),
        vec![],
    ));
    specs.push(ProcedureSpec::new(
        "bulkDelete",
        casing.routine_name(&format!("bulk_delete_{plural}")),
        vec![],
    ));
    specs
}
