use crate::mapping::SqlMapping;
use crate::schema::{field_overlay, overlay_of, table_identity, uses_generated_id_column, SqlTable};
use crate::spec::{is_pk_field, SpecTypeField};
use crate::sql_dialect;
use datasources::{
    AlterAction, AlterTableStatement, Column, ColumnDefault, CreateBulkCreateStoredProcedureStatement,
    CreateBulkDeleteStoredProcedureStatement, CreateBulkUpdateStoredProcedureStatement,
    CreateCreateStoredProcedureStatement, CreateDeleteStoredProcedureStatement, CreateIndexStatement,
    CreateReadStoredProcedureStatement, CreateTableStatement,
    CreateTriggerStatement, CreateUpdateStoredProcedureStatement, DropIndexStatement,
    DropRoutineStatement, DropTableStatement, DropTriggerStatement, ForeignKey,
    IdentityInsertStatement, InsertStatement, NotNull, Parameter, PrimaryKey, Result,
    RoutineParameter, TableConstraint, UniqueConstraint, Value,
};

pub fn create_table(dialect: &str, table: &SqlTable, mapping: &SqlMapping) -> Result<CreateTableStatement> {
    let keys = table_identity(table);
    let columns: Vec<Column> = table
        .fields
        .iter()
        .map(|field| column_of(dialect, mapping, table, field))
        .collect::<Result<_>>()?;
    let uniques: Vec<TableConstraint> = table
        .fields
        .iter()
        .filter(|field| field_overlay(table, &field.name).and_then(|o| o.is_unique) == Some(true) || field.name == "uuid")
        .map(|field| {
            let name = sql_dialect::q(
                dialect,
                &mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "unique_constraint"]),
            )?;
            let col = sql_dialect::q(dialect, &mapping.column_name(table, &field.name))?;
            Ok(UniqueConstraint::new(name, &[col.as_str()]))
        })
        .collect::<Result<_>>()?;
    let foreign_keys: Vec<TableConstraint> = table
        .fields
        .iter()
        .filter(|field| field.references.is_some() || field.reference_pair.is_some())
        .map(|field| {
            let reference = mapping.resolve_reference(field)?;
            let name = sql_dialect::q(
                dialect,
                &mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "foreign_key"]),
            )?;
            let col = sql_dialect::q(dialect, &mapping.column_name(table, &field.name))?;
            let table_name = sql_dialect::q(dialect, &reference.0)?;
            let ref_col = sql_dialect::q(dialect, &reference.1)?;
            Ok(ForeignKey::new(
                name,
                &[col.as_str()],
                table_name,
                &[ref_col.as_str()],
                None,
                None,
            ))
        })
        .collect::<Result<_>>()?;
    let overlay = overlay_of(table);
    let pk: Vec<TableConstraint> = if keys.len() > 1 {
        let name = sql_dialect::q(dialect, &mapping.constraint_name(table, &["primary_key"]))?;
        let cols: Vec<String> = keys
            .iter()
            .map(|key| sql_dialect::q(dialect, &mapping.column_name(table, key)))
            .collect::<Result<_>>()?;
        let refs: Vec<&str> = cols.iter().map(String::as_str).collect();
        vec![PrimaryKey::new(name, &refs)]
    } else if keys.len() == 1
        && table
            .fields
            .iter()
            .any(|field| is_pk_field(field, &table.as_type(), Some(&overlay)))
    {
        let name = sql_dialect::q(dialect, &mapping.constraint_name(table, &["primary_key"]))?;
        let col = sql_dialect::q(dialect, &mapping.column_name(table, &keys[0]))?;
        vec![PrimaryKey::new(name, &[col.as_str()])]
    } else {
        vec![]
    };
    let mut constraints = pk;
    constraints.extend(uniques);
    constraints.extend(foreign_keys);
    Ok(CreateTableStatement::new(
        sql_dialect::q(dialect, &table.table_name)?,
        &columns,
        &constraints,
    ))
}

pub fn create_index(
    dialect: &str,
    mapping: &SqlMapping,
    table: &SqlTable,
    index: &crate::spec::DatasourceIndex,
) -> Result<CreateIndexStatement> {
    let cols: Vec<String> = index
        .fields
        .iter()
        .map(|column| sql_dialect::q(dialect, &mapping.column_name(table, column)))
        .collect::<Result<_>>()?;
    let refs: Vec<&str> = cols.iter().map(String::as_str).collect();
    Ok(CreateIndexStatement::new(
        sql_dialect::q(dialect, &mapping.constraint_name(table, &[&index.name, "index"]))?,
        sql_dialect::q(dialect, &table.table_name)?,
        &refs,
        index.is_unique,
    ))
}

pub fn drop_table(dialect: &str, table: &str) -> Result<DropTableStatement> {
    Ok(DropTableStatement::new(sql_dialect::q(dialect, table)?))
}

pub fn drop_index(dialect: &str, name: &str, table: &str) -> Result<DropIndexStatement> {
    Ok(DropIndexStatement::new(
        sql_dialect::q(dialect, name)?,
        sql_dialect::q(dialect, table)?,
    ))
}

pub fn drop_trigger(dialect: &str, name: &str, table: &str) -> Result<DropTriggerStatement> {
    Ok(DropTriggerStatement::new(
        sql_dialect::q(dialect, name)?,
        sql_dialect::q(dialect, table)?,
    ))
}

pub fn column(dialect: &str, mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<Column> {
    column_of(dialect, mapping, table, field)
}

pub fn unique_of(
    dialect: &str,
    mapping: &SqlMapping,
    table: &SqlTable,
    field: &SpecTypeField,
) -> Result<TableConstraint> {
    let name = sql_dialect::q(
        dialect,
        &mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "unique_constraint"]),
    )?;
    let col = sql_dialect::q(dialect, &mapping.column_name(table, &field.name))?;
    Ok(UniqueConstraint::new(name, &[col.as_str()]))
}

pub fn foreign_key_of(
    dialect: &str,
    mapping: &SqlMapping,
    table: &SqlTable,
    field: &SpecTypeField,
) -> Result<TableConstraint> {
    let reference = mapping.resolve_reference(field)?;
    let name = sql_dialect::q(
        dialect,
        &mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "foreign_key"]),
    )?;
    let col = sql_dialect::q(dialect, &mapping.column_name(table, &field.name))?;
    let table_name = sql_dialect::q(dialect, &reference.0)?;
    let ref_col = sql_dialect::q(dialect, &reference.1)?;
    Ok(ForeignKey::new(
        name,
        &[col.as_str()],
        table_name,
        &[ref_col.as_str()],
        None,
        None,
    ))
}

pub fn alter_table(table: impl AsRef<str>, action: AlterAction) -> AlterTableStatement {
    AlterTableStatement::new(table, action)
}

pub fn create_trigger(dialect: &str, table: &SqlTable, mapping: &SqlMapping, key: &str) -> Result<CreateTriggerStatement> {
    Ok(CreateTriggerStatement::new(
        sql_dialect::q(dialect, &table.table_name)?,
        sql_dialect::q(dialect, &mapping.trigger_name(table))?,
        sql_dialect::q(dialect, &mapping.column_name(table, "updated"))?,
        sql_dialect::q(dialect, &mapping.column_name(table, key))?,
        sql_dialect::driver_of(dialect)?
            .default_value_converter
            .convert("datetime", "UtcNow")?
            .unwrap_or_default(),
    ))
}

pub fn seed_insert(
    table: &SqlTable,
    mapping: &SqlMapping,
    columns: &[String],
    values: &[String],
    dialect: &str,
) -> Result<InsertStatement> {
    let fields: Vec<Parameter> = columns
        .iter()
        .enumerate()
        .map(|(i, column)| {
            Ok(Parameter::new(
                sql_dialect::q(dialect, &mapping.column_name(table, column))?,
                sql_value(values.get(i).map(String::as_str).unwrap_or("NULL")),
            ))
        })
        .collect::<Result<_>>()?;
    Ok(InsertStatement::new(sql_dialect::q(dialect, &table.table_name)?, &fields))
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

fn column_of(dialect: &str, mapping: &SqlMapping, table: &SqlTable, field: &SpecTypeField) -> Result<Column> {
    let converter_field = field.converter_field();
    let generated = uses_generated_id_column(table, field);
    let identity = generated && field.type_name != "uuid" && field.type_name != "string";
    let named = mapping.constraint_name(table, &[&mapping.field_stem(table, &field.name), "default_constraint"]);
    let utc_now = sql_dialect::driver_of(dialect)?
        .default_value_converter
        .convert("datetime", "UtcNow")?;
    let new_id = sql_dialect::driver_of(dialect)?
        .default_value_converter
        .convert("uuid", "NewId")?;
    let from_spec = sql_dialect::sql_default(dialect, &converter_field)?;
    let default_expr = from_spec
        .or_else(|| {
            if field.name == "created" || field.name == "updated" {
                utc_now.clone()
            } else {
                None
            }
        })
        .or_else(|| {
            if field.name == "uuid" || generated && field.type_name == "uuid" {
                new_id.clone()
            } else {
                None
            }
        });
    let keys = table_identity(table);
    let mut column = Column::new(
        sql_dialect::q(dialect, &mapping.column_name(table, &field.name))?,
        sql_dialect::map_column_type(dialect, &converter_field)?,
    );
    if let Some(expr) = default_expr {
        let quoted = sql_dialect::q(dialect, &named)?;
        column = column.default(ColumnDefault::new(expr, Some(quoted.as_str())));
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
