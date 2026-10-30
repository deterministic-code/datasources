use crate::mapping::SqlMapping;
use crate::schema::{
    field_overlay, has_audit_columns, is_generated_identity, overlay_of, table_identity, SqlTable,
};
use crate::spec::{pk_name, DatasourceIndex, SpecTypeField};
use crate::table_statements;
use datasources::{
    AlterAction, AlterTableStatement, Column, CreateIndexStatement, CreateTableStatement,
    CreateTriggerStatement, DatasourceError, DropIndexStatement, DropTableStatement,
    DropTriggerStatement, ForeignKeyStatement, Result, Statement, TableConstraint,
};

pub enum DdlStatement {
    CreateTable(CreateTableStatement),
    CreateIndex(CreateIndexStatement),
    CreateTrigger(CreateTriggerStatement),
    AlterTable(AlterTableStatement),
    DropIndex(DropIndexStatement),
    DropTrigger(DropTriggerStatement),
    DropTable(DropTableStatement),
    ForeignKey(ForeignKeyStatement),
}

impl DdlStatement {
    pub fn as_statement(&self) -> &dyn Statement {
        match self {
            Self::CreateTable(statement) => statement,
            Self::CreateIndex(statement) => statement,
            Self::CreateTrigger(statement) => statement,
            Self::AlterTable(statement) => statement,
            Self::DropIndex(statement) => statement,
            Self::DropTrigger(statement) => statement,
            Self::DropTable(statement) => statement,
            Self::ForeignKey(statement) => statement,
        }
    }

    pub fn is_create_table(&self) -> bool {
        matches!(self, Self::CreateTable(_))
    }
}

pub fn of(
    dialect: &str,
    before_tables: &[SqlTable],
    before_map: &SqlMapping,
    after_tables: &[SqlTable],
    after_map: &SqlMapping,
) -> Result<Vec<DdlStatement>> {
    let previous: std::collections::HashMap<_, _> =
        before_tables.iter().map(|table| (table.name.as_str(), table)).collect();
    let next: std::collections::HashMap<_, _> =
        after_tables.iter().map(|table| (table.name.as_str(), table)).collect();
    let mut out = Vec::new();
    for table in before_tables.iter().rev().filter(|table| !next.contains_key(table.name.as_str())) {
        out.extend(drop_table(dialect, table, before_map)?);
    }
    for table in after_tables.iter().filter(|table| previous.contains_key(table.name.as_str())) {
        out.extend(diff_table(
            dialect,
            previous[table.name.as_str()],
            table,
            before_map,
            after_map,
        )?);
    }
    for table in after_tables.iter().filter(|table| !previous.contains_key(table.name.as_str())) {
        out.extend(create_table(dialect, table, after_map)?);
    }
    Ok(out)
}

fn create_table(dialect: &str, table: &SqlTable, mapping: &SqlMapping) -> Result<Vec<DdlStatement>> {
    let mut out = vec![DdlStatement::CreateTable(table_statements::create_table(
        table, mapping,
    )?)];
    for index in &table.indexes {
        out.push(DdlStatement::CreateIndex(table_statements::create_index(
            mapping, table, index,
        )));
    }
    if has_audit_columns(table) {
        out.push(DdlStatement::CreateTrigger(trigger(dialect, table, mapping)?));
    }
    Ok(out)
}

fn drop_table(dialect: &str, table: &SqlTable, mapping: &SqlMapping) -> Result<Vec<DdlStatement>> {
    let mut out = Vec::new();
    if has_audit_columns(table) {
        out.push(DdlStatement::DropTrigger(drop_trigger(dialect, table, mapping)?));
    }
    for index in table.indexes.iter().rev() {
        out.push(DdlStatement::DropIndex(drop_index(dialect, mapping, table, index)?));
    }
    out.push(DdlStatement::DropTable(table_statements::drop_table(&table.table_name)));
    Ok(out)
}

fn diff_table(
    dialect: &str,
    before: &SqlTable,
    after: &SqlTable,
    before_map: &SqlMapping,
    after_map: &SqlMapping,
) -> Result<Vec<DdlStatement>> {
    if identity_of(before) != identity_of(after) {
        return Err(DatasourceError::message(format!(
            "{}: primary key or identity change is not supported",
            after.name
        )));
    }
    let before_fields: std::collections::HashMap<_, _> =
        before.fields.iter().map(|field| (field.name.as_str(), field)).collect();
    let after_fields: std::collections::HashMap<_, _> =
        after.fields.iter().map(|field| (field.name.as_str(), field)).collect();
    let before_indexes: std::collections::HashMap<_, _> =
        before.indexes.iter().map(|index| (index.name.as_str(), index)).collect();
    let after_indexes: std::collections::HashMap<_, _> =
        after.indexes.iter().map(|index| (index.name.as_str(), index)).collect();
    let mut out = Vec::new();

    for index in &before.indexes {
        if after_indexes.get(index.name.as_str()).map(|kept| index_key(kept)) != Some(index_key(index)) {
            out.push(DdlStatement::DropIndex(drop_index(dialect, before_map, before, index)?));
        }
    }

    for field in &before.fields {
        if unique_field(before, field)
            && after_fields
                .get(field.name.as_str())
                .map(|kept| !unique_field(after, kept))
                .unwrap_or(true)
        {
            out.push(DdlStatement::AlterTable(AlterTableStatement::new(
                &before.table_name,
                AlterAction::DropConstraint {
                    name: constraint_name(&table_statements::unique_of(before_map, before, field))
                        .into(),
                },
            )));
        }
        if has_refs(field)
            && after_fields
                .get(field.name.as_str())
                .map(|next| refs_of(next) != refs_of(field))
                .unwrap_or(true)
        {
            out.push(DdlStatement::ForeignKey(table_statements::drop_foreign_key(
                before_map, before, field,
            )?));
        }
    }

    if has_audit_columns(before) && !has_audit_columns(after) {
        out.push(DdlStatement::DropTrigger(drop_trigger(dialect, before, before_map)?));
    }

    for field in before.fields.iter().filter(|field| !after_fields.contains_key(field.name.as_str())) {
        let column = table_statements::column(before_map, before, field)?;
        out.push(DdlStatement::AlterTable(AlterTableStatement::new(
            &before.table_name,
            AlterAction::DropColumn { name: column.name },
        )));
    }

    let table_name = after.table_name.clone();
    if before.table_name != after.table_name {
        out.push(DdlStatement::AlterTable(AlterTableStatement::new(
            &before.table_name,
            AlterAction::RenameTable {
                to: after.table_name.clone(),
            },
        )));
    }

    for field in after.fields.iter().filter(|field| before_fields.contains_key(field.name.as_str())) {
        let previous = table_statements::column(before_map, before, before_fields[field.name.as_str()])?;
        let next = table_statements::column(after_map, after, field)?;
        if previous.name != next.name {
            out.push(DdlStatement::AlterTable(AlterTableStatement::new(
                &table_name,
                AlterAction::RenameColumn {
                    from: previous.name.clone(),
                    to: next.name.clone(),
                },
            )));
        }
        if (Column {
            name: next.name.clone(),
            ..previous
        }) != next
        {
            out.push(DdlStatement::AlterTable(AlterTableStatement::new(
                &table_name,
                AlterAction::AlterColumn(next),
            )));
        }
    }

    for field in after.fields.iter().filter(|field| !before_fields.contains_key(field.name.as_str())) {
        out.push(DdlStatement::AlterTable(AlterTableStatement::new(
            &table_name,
            AlterAction::AddColumn(table_statements::column(after_map, after, field)?),
        )));
    }

    for field in &after.fields {
        if unique_field(after, field)
            && before_fields
                .get(field.name.as_str())
                .map(|previous| !unique_field(before, previous))
                .unwrap_or(true)
        {
            out.push(DdlStatement::AlterTable(AlterTableStatement::new(
                &table_name,
                AlterAction::AddConstraint(table_statements::unique_of(after_map, after, field)),
            )));
        }
        if has_refs(field)
            && before_fields
                .get(field.name.as_str())
                .map(|prior| refs_of(prior) != refs_of(field))
                .unwrap_or(true)
        {
            out.push(DdlStatement::ForeignKey(table_statements::add_foreign_key(
                after_map, after, field,
            )?));
        }
    }

    for index in &after.indexes {
        if before_indexes.get(index.name.as_str()).map(|kept| index_key(kept)) != Some(index_key(index)) {
            out.push(DdlStatement::CreateIndex(table_statements::create_index(
                after_map, after, index,
            )));
        }
    }

    if !has_audit_columns(before) && has_audit_columns(after) {
        out.push(DdlStatement::CreateTrigger(trigger(dialect, after, after_map)?));
    }
    Ok(out)
}

fn trigger(_dialect: &str, table: &SqlTable, mapping: &SqlMapping) -> Result<CreateTriggerStatement> {
    Ok(table_statements::create_trigger(
        table,
        mapping,
        &pk_name(&table.as_type(), Some(&overlay_of(table))),
    ))
}

fn drop_trigger(_dialect: &str, table: &SqlTable, mapping: &SqlMapping) -> Result<DropTriggerStatement> {
    Ok(table_statements::drop_trigger(&mapping.trigger_name(table), &table.table_name))
}

fn drop_index(
    _dialect: &str,
    mapping: &SqlMapping,
    table: &SqlTable,
    index: &DatasourceIndex,
) -> Result<DropIndexStatement> {
    Ok(table_statements::drop_index(
        &mapping.constraint_name(table, &[&index.name, "index"]),
        &table.table_name,
    ))
}

fn unique_field(table: &SqlTable, field: &SpecTypeField) -> bool {
    field_overlay(table, &field.name).and_then(|overlay| overlay.is_unique) == Some(true)
        || field.name == "uuid"
}

fn index_key(index: &DatasourceIndex) -> String {
    format!("{}:{:?}:{}", index.name, index.fields, index.is_unique)
}

fn identity_of(table: &SqlTable) -> String {
    format!("{:?}:{}", table_identity(table), is_generated_identity(table))
}

fn has_refs(field: &SpecTypeField) -> bool {
    field.references.is_some() || field.reference_pair.is_some()
}

fn refs_of(field: &SpecTypeField) -> String {
    format!("{:?}:{:?}", field.references, field.reference_pair)
}

fn constraint_name(constraint: &TableConstraint) -> String {
    match constraint {
        TableConstraint::PrimaryKey(item) => item.name.clone(),
        TableConstraint::ForeignKey(item) => item.name.clone(),
        TableConstraint::Unique(item) => item.name.clone(),
        TableConstraint::Check(item) => item.name.clone(),
    }
}
