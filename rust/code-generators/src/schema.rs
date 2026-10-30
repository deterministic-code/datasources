use crate::casing::PackCasing;
use datasources::{CreateTableStatement, StatementNaming};
use crate::spec::{
    datasource_types_of, dictionary_unique_columns, identity_columns, persisted_column_fields,
    table_by_name, DatasourceFieldOverlay, DatasourceIndex, DatasourceTable, ParsedSpec, SpecType,
    SpecTypeField,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct SqlTable {
    pub name: String,
    pub tags: Vec<String>,
    pub kind: String,
    pub inherits: Option<String>,
    pub ids: Option<Vec<String>>,
    pub fields: Vec<SpecTypeField>,
    pub indexes: Vec<DatasourceIndex>,
    pub unique_index_fields: Vec<String>,
    pub mapping: Option<String>,
    pub overlays: Vec<DatasourceFieldOverlay>,
    pub table_name: String,
}

impl SqlTable {
    pub fn as_type(&self) -> SpecType {
        SpecType {
            name: self.name.clone(),
            tags: self.tags.clone(),
            kind: self.kind.clone(),
            inherits: self.inherits.clone(),
            union: None,
            mapping: None,
            extract: None,
            remove_fields: None,
            ids: self.ids.clone(),
            fields: self.fields.clone(),
        }
    }
}

pub fn use_stored_procedures(settings: &HashMap<String, String>) -> bool {
    settings.get("datasource.use_stored_procedures").map(String::as_str) == Some("true")
}

pub fn overlay_of(table: &SqlTable) -> DatasourceTable {
    DatasourceTable {
        name: table.name.clone(),
        mapping: table.mapping.clone(),
        fields: table.overlays.clone(),
        indexes: table.indexes.clone(),
        unique_index_fields: table.unique_index_fields.clone(),
        use_optimistic_concurrency: None,
    }
}

pub fn field_overlay<'a>(table: &'a SqlTable, field_name: &str) -> Option<&'a DatasourceFieldOverlay> {
    table.overlays.iter().find(|overlay| overlay.name == field_name)
}

pub fn table_identity(table: &SqlTable) -> Vec<String> {
    let keys = identity_columns(Some(&table.as_type()), Some(&overlay_of(table)));
    if keys.len() > 1 {
        return if keys.iter().all(|col| table.fields.iter().any(|f| f.name == *col)) {
            keys
        } else {
            vec![]
        };
    }
    let Some(col) = keys.first() else {
        return vec![];
    };
    if table.fields.iter().any(|f| f.name == *col) || table.inherits.as_deref() == Some("set") {
        vec![col.clone()]
    } else {
        vec![]
    }
}

pub fn is_generated_identity(table: &SqlTable) -> bool {
    let keys = table_identity(table);
    keys.len() == 1 && field_overlay(table, &keys[0]).and_then(|o| o.is_fixed_id) != Some(true)
}

pub fn uses_generated_id_column(table: &SqlTable, field: &SpecTypeField) -> bool {
    field.is_id && is_generated_identity(table)
}

pub fn from_spec(spec: &ParsedSpec) -> Vec<SqlTable> {
    let overlays = table_by_name(spec);
    let by_name: HashMap<String, SpecType> = spec
        .expanded_types
        .iter()
        .map(|t| (t.name.clone(), t.clone()))
        .collect();
    datasource_types_of(spec)
        .into_iter()
        .map(|type_| {
            let table = overlays.get(&type_.name);
            let unique_cols = dictionary_unique_columns(&type_, &by_name);
            let mut indexes = table.map(|t| t.indexes.clone()).unwrap_or_default();
            if !unique_cols.is_empty()
                && !indexes.iter().any(|idx| {
                    idx.is_unique && idx.fields.len() == unique_cols.len() && idx.fields == unique_cols
                })
            {
                indexes.push(DatasourceIndex {
                    name: "owner_key".into(),
                    fields: unique_cols,
                    is_unique: true,
                });
            }
            let fields = persisted_column_fields(&type_, &by_name);
            SqlTable {
                name: type_.name,
                tags: type_.tags,
                kind: type_.kind,
                inherits: type_.inherits,
                ids: type_.ids,
                fields,
                indexes,
                unique_index_fields: table.map(|t| t.unique_index_fields.clone()).unwrap_or_default(),
                mapping: table.and_then(|t| t.mapping.clone()),
                overlays: table.map(|t| t.fields.clone()).unwrap_or_default(),
                table_name: String::new(),
            }
        })
        .collect()
}

pub fn has_audit_columns(table: &SqlTable) -> bool {
    table.fields.iter().any(|f| f.name == "created") && table.fields.iter().any(|f| f.name == "updated")
}

pub fn table_stem(table: &SqlTable) -> String {
    table.mapping.clone().unwrap_or_else(|| table.name.clone())
}

pub fn field_stem(table: &SqlTable, field_name: &str) -> String {
    field_overlay(table, field_name)
        .and_then(|o| o.mapping.clone())
        .unwrap_or_else(|| field_name.into())
}

pub fn table_of<'a>(tables: &'a [SqlTable], logical_name: &str) -> datasources::Result<&'a SqlTable> {
    tables
        .iter()
        .find(|table| table.name == logical_name)
        .ok_or_else(|| {
            datasources::DatasourceError::message(format!(
                "foreign key references unknown table \"{logical_name}\""
            ))
        })
}

pub fn resolve_reference(
    tables: &[SqlTable],
    naming: &StatementNaming,
    field: &SpecTypeField,
) -> datasources::Result<(String, String)> {
    let (logical_table, logical_col) = crate::spec::reference_target(field);
    let target = table_of(tables, &logical_table)?;
    Ok((
        CreateTableStatement::table_name(&table_stem(target), naming),
        CreateTableStatement::column_name(&field_stem(target, &logical_col), naming),
    ))
}

pub fn build_live(types: &[SqlTable], naming: &StatementNaming) -> Vec<SqlTable> {
    topo(
        types
            .iter()
            .map(|t| {
                let mut next = t.clone();
                next.table_name = CreateTableStatement::table_name(&table_stem(t), naming);
                next
            })
            .collect(),
    )
}

fn topo(tables: Vec<SqlTable>) -> Vec<SqlTable> {
    let names: HashSet<String> = tables.iter().map(|t| t.name.clone()).collect();
    let deps: HashMap<String, HashSet<String>> = tables
        .iter()
        .map(|t| {
            let deps = t
                .fields
                .iter()
                .map(|f| crate::spec::reference_target(f).0)
                .filter(|dep| names.contains(dep) && dep != &t.name && !dep.is_empty())
                .collect();
            (t.name.clone(), deps)
        })
        .collect();
    let mut output = Vec::new();
    let mut done: HashSet<String> = HashSet::new();
    let mut pending = tables;
    while !pending.is_empty() {
        let idx = pending.iter().position(|t| deps[&t.name].iter().all(|d| done.contains(d)));
        match idx {
            None => {
                output.extend(pending);
                break;
            }
            Some(idx) => {
                let t = pending.remove(idx);
                done.insert(t.name.clone());
                output.push(t);
            }
        }
    }
    output
}

pub fn pack_of(settings: &HashMap<String, String>) -> datasources::Result<PackCasing> {
    PackCasing::of(settings)
}
