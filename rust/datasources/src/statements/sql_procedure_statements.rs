use super::create_bulk_create_stored_procedure_statement::CreateBulkCreateStoredProcedureStatement;
use super::create_bulk_delete_stored_procedure_statement::CreateBulkDeleteStoredProcedureStatement;
use super::create_bulk_update_stored_procedure_statement::CreateBulkUpdateStoredProcedureStatement;
use super::create_create_stored_procedure_statement::CreateCreateStoredProcedureStatement;
use super::create_delete_stored_procedure_statement::CreateDeleteStoredProcedureStatement;
use super::create_read_stored_procedure_statement::CreateReadStoredProcedureStatement;
use super::create_update_stored_procedure_statement::CreateUpdateStoredProcedureStatement;
use super::procedure_source::ProcedureSource;
use super::routine_parameter::RoutineParameter;
use super::statement::Statement;

pub fn sql_procedure_statements_of(source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
    let name = |stem: &str| (source.routine_name)(stem);
    let plural = &source.plural;
    let payload = RoutineParameter::new("payload", "payload");
    let mut statements: Vec<Box<dyn Statement>> = vec![
        Box::new(CreateCreateStoredProcedureStatement::new(
            name(&format!("create_{plural}")),
            &source.table,
            source.columns.clone(),
            source.create_parameters.clone(),
            &source.key,
            &source.key_type,
            source.writable.clone(),
            source.generated_key,
            source.audit_updated.clone(),
        )),
        Box::new(CreateReadStoredProcedureStatement::new(
            name(&format!("read_{plural}")),
            &source.table,
            source.columns.clone(),
            Vec::new(),
            &source.key,
            &source.key_type,
            source.writable.clone(),
            Vec::new(),
            source.generated_key,
            source.audit_updated.clone(),
        )),
    ];
    for group in &source.by_groups {
        let suffix = format!("_by_{}", group.logical.join("_"));
        statements.push(Box::new(CreateReadStoredProcedureStatement::new(
            name(&format!("read_{plural}{suffix}")),
            &source.table,
            source.columns.clone(),
            group.parameters.clone(),
            &source.key,
            &source.key_type,
            source.writable.clone(),
            group.columns.clone(),
            source.generated_key,
            source.audit_updated.clone(),
        )));
        statements.push(Box::new(CreateUpdateStoredProcedureStatement::new(
            name(&format!("update_{plural}{suffix}")),
            &source.table,
            source.columns.clone(),
            group.update_parameters.clone(),
            &source.key,
            &source.key_type,
            source.writable.clone(),
            group.columns.clone(),
            source.generated_key,
            source.audit_updated.clone(),
        )));
        statements.push(Box::new(CreateDeleteStoredProcedureStatement::new(
            name(&format!("delete_{plural}{suffix}")),
            &source.table,
            source.columns.clone(),
            group.parameters.clone(),
            &source.key,
            &source.key_type,
            source.writable.clone(),
            group.columns.clone(),
            source.generated_key,
            source.audit_updated.clone(),
        )));
    }
    statements.push(Box::new(CreateBulkCreateStoredProcedureStatement::new(
        name(&format!("bulk_create_{plural}")),
        &source.table,
        source.create_columns.clone(),
        payload.clone(),
        &source.key,
        &source.key_type,
    )));
    statements.push(Box::new(CreateBulkUpdateStoredProcedureStatement::new(
        name(&format!("bulk_update_{plural}")),
        &source.table,
        source.update_columns.clone(),
        payload.clone(),
        &source.key,
        &source.key_type,
    )));
    statements.push(Box::new(CreateBulkDeleteStoredProcedureStatement::new(
        name(&format!("bulk_delete_{plural}")),
        &source.table,
        source.typed_columns.clone(),
        payload,
        &source.key,
        &source.key_type,
    )));
    statements
}

pub fn procedure_name_of(statement: &dyn Statement) -> String {
    if let Some(s) = statement.as_any().downcast_ref::<CreateCreateStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateReadStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateUpdateStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateDeleteStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateBulkCreateStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateBulkUpdateStoredProcedureStatement>() {
        return s.name().to_string();
    }
    if let Some(s) = statement.as_any().downcast_ref::<CreateBulkDeleteStoredProcedureStatement>() {
        return s.name().to_string();
    }
    panic!("procedure statement has no name");
}
