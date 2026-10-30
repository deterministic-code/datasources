use crate::mapping::SqlMapping;
use crate::schema::{field_overlay, overlay_of, table_identity, uses_generated_id_column, SqlTable};
use crate::spec::{is_pk_field, SpecTypeField};
use datasources::{
    AlterAction, AlterTableStatement, Column, ColumnDefault, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement, CreateIndexStatement,
    CreateReadStoredProcedureStatement, CreateTableStatement, CreateTriggerStatement,
    CreateUpdateStoredProcedureStatement, DropIndexStatement, DropRoutineStatement, DropTableStatement,
    DropTriggerStatement, ForeignKey, ForeignKeyStatement, IdentityInsertStatement, InsertStatement,
    NotNull, Parameter, PrimaryKey, Result, RoutineParameter, TableConstraint, UniqueConstraint, Value,
};

pub fn create_table(table: &SqlTable, mapping: &SqlMapping) -> Result<CreateTableStatement> {
    let keys = table_identity(table);
    let columns: Vec<Column> = table
        .fields
        .iter()
        .map(|field| column_of(mapping, table, field))
        .collect::<Result<_>>()?;
    let uniques: Vec<TableConstraint> = table
        .fields
        .iter()
        .filter(|field| field_overlay(table, &field.name).and_then(|o| o.is_unique) == Some(true) || field.name == "uuid")
        .map(|field| {
            let name = mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "unique_constraint"]);
            let col = mapping.column_name(table, &field.name);
            UniqueConstraint::new(name, &[col.as_str()])
        })
        .collect();
    let foreign_keys: Vec<TableConstraint> = table
        .fields
        .iter()
        .filter(|field| field.references.is_some() || field.reference_pair.is_some())
        .map(|field| foreign_key_of(mapping, table, field))
        .collect::<Result<_>>()?;
    let overlay = overlay_of(table);
    let pk: Vec<TableConstraint> = if keys.len() > 1 {
        let cols: Vec<String> = keys.iter().map(|key| mapping.column_name(table, key)).collect();
        let refs: Vec<&str> = cols.iter().map(String::as_str).collect();
        vec![PrimaryKey::new(mapping.constraint_name(table, &["primary_key"]), &refs)]
    } else if keys.len() == 1
        && table
            .fields
            .iter()
            .any(|field| is_pk_field(field, &table.as_type(), Some(&overlay)))
    {
        let col = mapping.column_name(table, &keys[0]);
        vec![PrimaryKey::new(
            mapping.constraint_name(table, &["primary_key"]),
            &[col.as_str()],
        )]
    } else {
        vec![]
    };
    let mut constraints = pk;
    constraints.extend(uniques);
    constraints.extend(foreign_keys);
    Ok(CreateTableStatement::new(table.table_name.clone(), &columns, &constraints))
}

pub fn create_index(
    mapping: &SqlMapping,
    table: &SqlTable,
    index: &crate::spec::DatasourceIndex,
) -> CreateIndexStatement {
    let cols: Vec<String> = index
        .fields
        .iter()
        .map(|column| mapping.column_name(table, column))
        .collect();
    let refs: Vec<&str> = cols.iter().map(String::as_str).collect();
    CreateIndexStatement::new(
        mapping.constraint_name(table, &[&index.name, "index"]),
        table.table_name.clone(),
        &refs,
        index.is_unique,
    )
}

pub fn drop_table(table: &str) -> DropTableStatement {
    DropTableStatement::new(table)
}

pub fn drop_index(name: &str, table: &str) -> DropIndexStatement {
    DropIndexStatement::new(name, table)
}

pub fn drop_trigger(name: &str, table: &str) -> DropTriggerStatement {
    DropTriggerStatement::new(name, table)
}

pub fn column(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<Column> {
    column_of(mapping, table, field)
}

pub fn unique_of(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> TableConstraint {
    let name = mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "unique_constraint"]);
    let col = mapping.column_name(table, &field.name);
    UniqueConstraint::new(name, &[col.as_str()])
}

pub fn foreign_key_of(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<TableConstraint> {
    let reference = mapping.resolve_reference(field)?;
    let name = mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "foreign_key"]);
    let col = mapping.column_name(table, &field.name);
    Ok(ForeignKey::new(
        name,
        &[col.as_str()],
        reference.0,
        &[reference.1.as_str()],
        None,
        None,
    ))
}

pub fn add_foreign_key(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<ForeignKeyStatement> {
    let TableConstraint::ForeignKey(fk) = foreign_key_of(mapping, table, field)? else {
        return Err(datasources::DatasourceError::message("expected foreign key"));
    };
    Ok(ForeignKeyStatement::new(
        table.table_name.clone(),
        fk.name,
        fk.columns,
        fk.references_table,
        fk.references_columns,
        "add",
    ))
}

pub fn drop_foreign_key(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<ForeignKeyStatement> {
    let TableConstraint::ForeignKey(fk) = foreign_key_of(mapping, table, field)? else {
        return Err(datasources::DatasourceError::message("expected foreign key"));
    };
    Ok(ForeignKeyStatement::new(
        table.table_name.clone(),
        fk.name,
        Vec::new(),
        "",
        Vec::new(),
        "drop",
    ))
}

pub fn alter_table(table: impl AsRef<str>, action: AlterAction) -> AlterTableStatement {
    AlterTableStatement::new(table, action)
}

pub fn create_trigger(table: &SqlTable, mapping: &SqlMapping, key: &str) -> CreateTriggerStatement {
    CreateTriggerStatement::new(
        table.table_name.clone(),
        mapping.trigger_name(table),
        mapping.column_name(table, "updated"),
        mapping.column_name(table, key),
    )
}

pub fn seed_insert(
    table: &SqlTable,
    mapping: &SqlMapping,
    columns: &[String],
    values: &[String],
) -> InsertStatement {
    let fields: Vec<Parameter> = columns
        .iter()
        .enumerate()
        .map(|(i, column)| {
            Parameter::new(
                mapping.column_name(table, column),
                sql_value(values.get(i).map(String::as_str).unwrap_or("NULL")),
            )
        })
        .collect();
    InsertStatement::new(table.table_name.clone(), &fields)
}

pub fn identity_insert(
    table: &SqlTable,
    mapping: &SqlMapping,
    inserts: Vec<InsertStatement>,
    identity: bool,
) -> IdentityInsertStatement {
    let keys = table_identity(table);
    IdentityInsertStatement::new(
        table.table_name.clone(),
        mapping.column_name(table, keys.first().map(String::as_str).unwrap_or("id")),
        inserts,
        identity,
    )
}

pub fn drop_routine(name: impl AsRef<str>) -> DropRoutineStatement {
    DropRoutineStatement::new(name)
}

pub fn create_create_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
    writable: Vec<String>,
    generated_key: bool,
    audit_updated: Option<String>,
) -> CreateCreateStoredProcedureStatement {
    CreateCreateStoredProcedureStatement::new(
        name, table, columns, parameters, key, key_type, writable, generated_key, audit_updated,
    )
}

pub fn create_read_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
    writable: Vec<String>,
    by_properties: Vec<String>,
    generated_key: bool,
    audit_updated: Option<String>,
) -> CreateReadStoredProcedureStatement {
    CreateReadStoredProcedureStatement::new(
        name, table, columns, parameters, key, key_type, writable, by_properties, generated_key, audit_updated,
    )
}

pub fn create_update_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
    writable: Vec<String>,
    by_properties: Vec<String>,
    generated_key: bool,
    audit_updated: Option<String>,
) -> CreateUpdateStoredProcedureStatement {
    CreateUpdateStoredProcedureStatement::new(
        name, table, columns, parameters, key, key_type, writable, by_properties, generated_key, audit_updated,
    )
}

pub fn create_delete_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
    writable: Vec<String>,
    by_properties: Vec<String>,
    generated_key: bool,
    audit_updated: Option<String>,
) -> CreateDeleteStoredProcedureStatement {
    CreateDeleteStoredProcedureStatement::new(
        name, table, columns, parameters, key, key_type, writable, by_properties, generated_key, audit_updated,
    )
}

pub fn create_bulk_create_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<RoutineParameter>,
    payload: RoutineParameter,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
) -> CreateBulkCreateStoredProcedureStatement {
    CreateBulkCreateStoredProcedureStatement::new(name, table, columns, payload, key, key_type)
}

pub fn create_bulk_update_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<RoutineParameter>,
    payload: RoutineParameter,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
) -> CreateBulkUpdateStoredProcedureStatement {
    CreateBulkUpdateStoredProcedureStatement::new(name, table, columns, payload, key, key_type)
}

pub fn create_bulk_delete_stored_procedure(
    name: impl AsRef<str>,
    table: impl AsRef<str>,
    columns: Vec<RoutineParameter>,
    payload: RoutineParameter,
    key: impl AsRef<str>,
    key_type: impl AsRef<str>,
) -> CreateBulkDeleteStoredProcedureStatement {
    CreateBulkDeleteStoredProcedureStatement::new(name, table, columns, payload, key, key_type)
}

fn column_of(mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<Column> {
    let converter_field = field.converter_field();
    let generated = uses_generated_id_column(table, field);
    let named = mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "default_constraint"]);
    let from_spec = converter_field.default_value.clone();
    let identity = generated && from_spec.is_none();
    let keys = table_identity(table);
    let mut column = Column::new(
        mapping.column_name(table, &field.name),
        converter_field.type_name.clone(),
    )
    .field(converter_field);
    if let Some(expr) = from_spec {
        column = column.default(ColumnDefault::new(expr, Some(named.as_str())));
    }
    if !field.is_nullable || keys.iter().any(|k| k == &field.name) {
        column = column.not_null(NotNull::new(None));
    }
    if identity {
        column = column.identity();
    }
    Ok(column)
}

fn sql_value(sql: &str) -> Value {
    if sql == "NULL" {
        Value::Null
    } else {
        Value::Text(sql.into())
    }
}
