use crate::casing::PackCasing;
use crate::schema::{field_overlay, SqlTable};
use crate::spec::{reference_target, SpecTypeField};
use datasources::{DatasourceError, Result};
use std::collections::HashMap;

pub struct SqlMapping {
    casing: PackCasing,
    by_logical_name: HashMap<String, SqlTable>,
}

impl SqlMapping {
    pub fn new(casing: PackCasing, tables: &[SqlTable]) -> Self {
        Self {
            by_logical_name: tables.iter().map(|t| (t.name.clone(), t.clone())).collect(),
            casing,
        }
    }

    pub fn table_stem(&self, table: &SqlTable) -> String {
        table.mapping.clone().unwrap_or_else(|| table.name.clone())
    }

    pub fn field_stem(&self, table: &SqlTable, field_name: &str) -> String {
        field_overlay(table, field_name)
            .and_then(|o| o.mapping.clone())
            .unwrap_or_else(|| field_name.into())
    }

    pub fn table_name(&self, table: &SqlTable) -> String {
        self.casing.table_name(&self.table_stem(table))
    }

    pub fn column_name(&self, table: &SqlTable, field_name: &str) -> String {
        self.casing.column_name(&self.field_stem(table, field_name))
    }

    pub fn constraint_name(&self, table: &SqlTable, parts: &[&str]) -> String {
        self.casing.constraint_name(&self.table_stem(table), parts)
    }

    pub fn trigger_name(&self, table: &SqlTable) -> String {
        self.casing.trigger_name(&self.table_stem(table))
    }

    pub fn table_of(&self, logical_name: &str) -> Result<&SqlTable> {
        self.by_logical_name.get(logical_name).ok_or_else(|| {
            DatasourceError::message(format!("foreign key references unknown table \"{logical_name}\""))
        })
    }

    pub fn resolve_reference(&self, field: &SpecTypeField) -> Result<(String, String)> {
        let (logical_table, logical_col) = reference_target(field);
        let target = self.table_of(&logical_table)?;
        Ok((self.table_name(target), self.column_name(target, &logical_col)))
    }

    pub fn casing(&self) -> &PackCasing {
        &self.casing
    }
}
