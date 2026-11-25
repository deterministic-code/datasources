use crate::schema::table_stem;
use datasources::StatementNaming;
use crate::schema::{
    field_overlay, has_audit_columns, is_generated_identity, overlay_of, rebuild_table, table_identity, SqlTable,
};
use crate::spec::{pk_name, DatasourceIndex, SpecTypeField};
use crate::table_statements;
use datasources::{
    AlterAction, AlterTableStatement, Column, CreateIndexStatement, CreateTableStatement,
    CreateTriggerStatement, DatasourceError, DropIndexStatement, DropTableStatement,
    DropTriggerStatement, ForeignKeyStatement, RebuildColumn, RebuildTableStatement, Result, Statement,
    TableConstraint,
};
use std::collections::{HashMap, HashSet};

pub enum DdlStatement {
    CreateTable(CreateTableStatement),
    CreateIndex(CreateIndexStatement),
    CreateTrigger(CreateTriggerStatement),
    AlterTable(AlterTableStatement),
    DropIndex(DropIndexStatement),
    DropTrigger(DropTriggerStatement),
    DropTable(DropTableStatement),
    ForeignKey(ForeignKeyStatement),
    RebuildTable(RebuildTableStatement),
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
            Self::RebuildTable(statement) => statement,
        }
    }

    pub fn is_create_table(&self) -> bool {
        matches!(self, Self::CreateTable(_))
    }
}

pub fn of(
    dialect: &str,
    before_tables: &[SqlTable],
    before_map: &StatementNaming,
    after_tables: &[SqlTable],
    after_map: &StatementNaming,
    settings: &HashMap<String, String>,
) -> Result<Vec<DdlStatement>> {
    let previous_by_name: std::collections::HashMap<_, _> =
        before_tables.iter().map(|table| (table.name.as_str(), table)).collect();
    let next_by_name: std::collections::HashMap<_, _> =
        after_tables.iter().map(|table| (table.name.as_str(), table)).collect();
    let previous_by_table: std::collections::HashMap<_, _> =
        before_tables.iter().map(|table| (table.table_name.as_str(), table)).collect();
    let next_by_table: std::collections::HashMap<_, _> =
        after_tables.iter().map(|table| (table.table_name.as_str(), table)).collect();
    let mut out = Vec::new();
    for table in before_tables
        .iter()
        .rev()
        .filter(|table| prior_of(&next_by_name, &next_by_table, table).is_none())
    {
        out.extend(drop_table(dialect, table, before_map)?);
    }
    for table in after_tables.iter() {
        if let Some(prior) = prior_of(&previous_by_name, &previous_by_table, table) {
            out.extend(diff_table(
                dialect,
                prior,
                table,
                before_map,
                after_map,
                before_tables,
                after_tables,
                settings,
            )?);
        }
    }
    for table in after_tables
        .iter()
        .filter(|table| prior_of(&previous_by_name, &previous_by_table, table).is_none())
    {
        out.extend(create_table(dialect, table, after_map, after_tables)?);
    }
    Ok(out)
}

fn prior_of<'a>(
    by_name: &std::collections::HashMap<&str, &'a SqlTable>,
    by_table: &std::collections::HashMap<&str, &'a SqlTable>,
    table: &SqlTable,
) -> Option<&'a SqlTable> {
    by_name
        .get(table.name.as_str())
        .copied()
        .or_else(|| by_table.get(table.table_name.as_str()).copied())
}

fn create_table(dialect: &str, table: &SqlTable, naming: &StatementNaming, tables: &[SqlTable]) -> Result<Vec<DdlStatement>> {
    let mut out = vec![DdlStatement::CreateTable(table_statements::create_table(
        table, naming, tables,
    )?)];
    for index in &table.indexes {
        out.push(DdlStatement::CreateIndex(table_statements::create_index(
            naming, table, index,
        )));
    }
    if has_audit_columns(table) {
        out.push(DdlStatement::CreateTrigger(trigger(dialect, table, naming)?));
    }
    Ok(out)
}

fn drop_table(dialect: &str, table: &SqlTable, mapping: &StatementNaming) -> Result<Vec<DdlStatement>> {
    let mut out = Vec::new();
    if has_audit_columns(table) {
        out.push(DdlStatement::DropTrigger(drop_trigger(dialect, table, mapping)?));
    }
    out.push(DdlStatement::DropTable(table_statements::drop_table(&table.table_name)));
    Ok(out)
}

fn diff_table(
    dialect: &str,
    before: &SqlTable,
    after: &SqlTable,
    before_map: &StatementNaming,
    after_map: &StatementNaming,
    before_tables: &[SqlTable],
    after_tables: &[SqlTable],
    settings: &HashMap<String, String>,
) -> Result<Vec<DdlStatement>> {
    if identity_of(before) != identity_of(after) {
        return Err(DatasourceError::message(format!(
            "{}: primary key or identity change is not supported",
            after.name
        )));
    }
    if rebuild_table(dialect, settings) && needs_rebuild(before, after, before_map, after_map)? {
        return Ok(vec![rebuild(before, after, before_map, after_map, after_tables)?]);
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
    let mut dropped_foreign_keys = HashSet::new();
    let mut created_indexes = HashSet::new();

    for index in &before.indexes {
        if after_indexes.get(index.name.as_str()).map(|kept| index_key(kept)) == Some(index_key(index)) {
            continue;
        }
        if dialect == "mysql" {
            out.extend(mysql_index_drop(
                before,
                after,
                before_map,
                after_map,
                before_tables,
                &after_fields,
                &mut dropped_foreign_keys,
                &mut created_indexes,
                index,
            )?);
        }
        out.push(DdlStatement::DropIndex(drop_index(dialect, before_map, before, index)?));
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
        if has_refs(field) && !same_refs(field, &after_fields) && dropped_foreign_keys.insert(field.name.clone()) {
            out.push(DdlStatement::ForeignKey(table_statements::drop_foreign_key(
                before_map, before_tables, before, field,
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
                after_map, after_tables, after, field,
            )?));
        }
    }

    for index in &after.indexes {
        if before_indexes.get(index.name.as_str()).map(|kept| index_key(kept)) != Some(index_key(index))
            && created_indexes.insert(index.name.clone())
        {
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

fn needs_rebuild(
    before: &SqlTable,
    after: &SqlTable,
    before_map: &StatementNaming,
    after_map: &StatementNaming,
) -> Result<bool> {
    let before_fields: std::collections::HashMap<_, _> =
        before.fields.iter().map(|field| (field.name.as_str(), field)).collect();
    let after_fields: std::collections::HashMap<_, _> =
        after.fields.iter().map(|field| (field.name.as_str(), field)).collect();
    for field in after.fields.iter().filter(|field| before_fields.contains_key(field.name.as_str())) {
        let previous = table_statements::column(before_map, before, before_fields[field.name.as_str()])?;
        let next = table_statements::column(after_map, after, field)?;
        if (Column {
            name: next.name.clone(),
            ..previous
        }) != next
        {
            return Ok(true);
        }
    }
    Ok(before
        .fields
        .iter()
        .chain(after.fields.iter())
        .any(|field| {
            if !has_refs(field) {
                return false;
            }
            match (
                before_fields.get(field.name.as_str()),
                after_fields.get(field.name.as_str()),
            ) {
                (Some(prior), Some(next)) => refs_of(prior) != refs_of(next),
                _ => true,
            }
        }))
}

fn rebuild(
    before: &SqlTable,
    after: &SqlTable,
    before_map: &StatementNaming,
    after_map: &StatementNaming,
    after_tables: &[SqlTable],
) -> Result<DdlStatement> {
    let before_fields: std::collections::HashMap<_, _> =
        before.fields.iter().map(|field| (field.name.as_str(), field)).collect();
    let mut columns = Vec::new();
    for field in &after.fields {
        let next = table_statements::column(after_map, after, field)?;
        if let Some(prior) = before_fields.get(field.name.as_str()) {
            let previous = table_statements::column(before_map, before, prior)?;
            let mut column = RebuildColumn::copied(&next.name, &previous.name);
            if previous.type_name != next.type_name {
                column = column.with_cast(&next.type_name);
            }
            columns.push(column);
        } else {
            if next.not_null.is_some() && next.default.is_none() {
                return Err(DatasourceError::message(format!(
                    "{}.{}: new NOT NULL column with no default",
                    after.name, field.name
                )));
            }
            columns.push(match &next.default {
                Some(default) => RebuildColumn::literal(&next.name, &default.value),
                None => RebuildColumn::null(&next.name),
            });
        }
    }
    let indexes = after
        .indexes
        .iter()
        .map(|index| table_statements::create_index(after_map, after, index))
        .collect();
    let triggers = if has_audit_columns(after) {
        vec![trigger("", after, after_map)?]
    } else {
        vec![]
    };
    Ok(DdlStatement::RebuildTable(RebuildTableStatement::new(
        &before.table_name,
        table_statements::create_table(after, after_map, after_tables)?,
        columns,
        indexes,
        triggers,
    )))
}

fn trigger(_dialect: &str, table: &SqlTable, naming: &StatementNaming) -> Result<CreateTriggerStatement> {
    Ok(table_statements::create_trigger(
        table,
        naming,
        &pk_name(&table.as_type(), Some(&overlay_of(table))),
    ))
}

fn drop_trigger(_dialect: &str, table: &SqlTable, naming: &StatementNaming) -> Result<DropTriggerStatement> {
    Ok(table_statements::drop_trigger(
        &datasources::CreateTriggerStatement::name_of(&table_stem(table), naming),
        &table.table_name,
    ))
}

fn drop_index(
    _dialect: &str,
    naming: &StatementNaming,
    table: &SqlTable,
    index: &DatasourceIndex,
) -> Result<DropIndexStatement> {
    Ok(table_statements::drop_index(
        &index_physical(naming, table, index),
        &table.table_name,
    ))
}

fn index_physical(naming: &StatementNaming, table: &SqlTable, index: &DatasourceIndex) -> String {
    CreateIndexStatement::name_of(&table_stem(table), naming, &[&index.name, "index"])
}

fn same_refs(field: &SpecTypeField, after_fields: &HashMap<&str, &SpecTypeField>) -> bool {
    after_fields
        .get(field.name.as_str())
        .is_some_and(|next| refs_of(next) == refs_of(field))
}

fn index_supports(index: &DatasourceIndex, field: &SpecTypeField) -> bool {
    has_refs(field) && index.fields.first().is_some_and(|name| name == &field.name)
}

fn pk_supports(table: &SqlTable, field: &SpecTypeField) -> bool {
    table_identity(table).first().is_some_and(|name| name == &field.name)
}

fn supporting_index_name(naming: &StatementNaming, table: &SqlTable, dropped_physical: &str, field: &str) -> String {
    for candidate in [field.to_string(), format!("{field}_fk")] {
        let index = DatasourceIndex {
            name: candidate.clone(),
            fields: vec![field.to_string()],
            is_unique: false,
        };
        if index_physical(naming, table, &index) != dropped_physical {
            return candidate;
        }
    }
    format!("{field}_supporting")
}

fn mysql_index_drop(
    before: &SqlTable,
    after: &SqlTable,
    before_map: &StatementNaming,
    after_map: &StatementNaming,
    before_tables: &[SqlTable],
    after_fields: &HashMap<&str, &SpecTypeField>,
    dropped_foreign_keys: &mut HashSet<String>,
    created_indexes: &mut HashSet<String>,
    index: &DatasourceIndex,
) -> Result<Vec<DdlStatement>> {
    let dropped_physical = index_physical(before_map, before, index);
    let mut out = Vec::new();
    for field in before.fields.iter().filter(|item| index_supports(index, item)) {
        if !same_refs(field, after_fields) {
            if dropped_foreign_keys.insert(field.name.clone()) {
                out.push(DdlStatement::ForeignKey(table_statements::drop_foreign_key(
                    before_map, before_tables, before, field,
                )?));
            }
            continue;
        }
        let supporting: Vec<&DatasourceIndex> = after
            .indexes
            .iter()
            .filter(|item| {
                index_supports(item, field) && index_physical(after_map, after, item) != dropped_physical
            })
            .collect();
        let unique_stays = after_fields
            .get(field.name.as_str())
            .is_some_and(|kept| unique_field(after, kept));
        if pk_supports(after, field) || unique_stays || !supporting.is_empty() {
            for added in supporting {
                let already = before
                    .indexes
                    .iter()
                    .any(|prior| prior.name == added.name && index_key(prior) == index_key(added));
                if !already && created_indexes.insert(added.name.clone()) {
                    out.push(DdlStatement::CreateIndex(table_statements::create_index(
                        after_map, after, added,
                    )));
                }
            }
            continue;
        }
        let support = DatasourceIndex {
            name: supporting_index_name(after_map, after, &dropped_physical, &field.name),
            fields: vec![field.name.clone()],
            is_unique: false,
        };
        created_indexes.insert(support.name.clone());
        out.push(DdlStatement::CreateIndex(table_statements::create_index(
            after_map, after, &support,
        )));
    }
    Ok(out)
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
