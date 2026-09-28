use std::collections::HashMap;
use std::fs;

use datasources::{
    CheckConstraint, Column, ColumnDefault, CreateTableStatement, ForeignKey, NotNull, PrimaryKey,
    UniqueConstraint,
};
use serde::Deserialize;

use crate::repo_root;

#[derive(Debug, Deserialize, Clone)]
struct ColumnSpec {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    #[serde(default, rename = "primaryKey")]
    primary_key: bool,
    #[serde(default, rename = "notNull")]
    not_null: bool,
    default: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct ConstraintSpec {
    kind: String,
    name: String,
    #[serde(default)]
    columns: Vec<String>,
    references: Option<String>,
    #[serde(default, rename = "referencesColumns")]
    references_columns: Vec<String>,
    expression: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct TableSpec {
    name: String,
    #[serde(default, rename = "use")]
    include: Vec<String>,
    #[serde(default)]
    columns: Vec<ColumnSpec>,
    #[serde(default)]
    constraints: Vec<ConstraintSpec>,
}

#[derive(Debug, Deserialize)]
struct CatalogFile {
    fragments: HashMap<String, Vec<ColumnSpec>>,
    schemas: HashMap<String, Vec<TableSpec>>,
}

fn catalog() -> CatalogFile {
    serde_yaml::from_str(&fs::read_to_string(repo_root().join("specs/schemas.yaml")).expect("schemas.yaml"))
        .expect("schemas.yaml")
}

fn physical(logical: &str, rename: Option<&HashMap<String, String>>) -> String {
    rename
        .and_then(|names| names.get(logical).cloned())
        .unwrap_or_else(|| logical.to_string())
}

fn quote_all<Q: Fn(&str) -> String + ?Sized>(names: &[String], quote: &Q) -> Vec<String> {
    names.iter().map(|name| quote(name)).collect()
}

fn statement_of<Q>(
    table: &TableSpec,
    quote: &Q,
    rename: Option<&HashMap<String, String>>,
    fragments: &HashMap<String, Vec<ColumnSpec>>,
) -> CreateTableStatement
where
    Q: Fn(&str) -> String + ?Sized,
{
    let mut specs = Vec::new();
    for fragment in &table.include {
        specs.extend(
            fragments
                .get(fragment)
                .unwrap_or_else(|| panic!("unknown fragment {fragment}"))
                .clone(),
        );
    }
    specs.extend(table.columns.clone());
    let keys: Vec<&str> = specs
        .iter()
        .filter(|column| column.primary_key)
        .map(|column| column.name.as_str())
        .collect();
    let mut constraints = Vec::new();
    if !keys.is_empty() {
        let quoted = quote_all(&keys.iter().map(|key| (*key).to_string()).collect::<Vec<_>>(), quote);
        let borrowed: Vec<&str> = quoted.iter().map(String::as_str).collect();
        constraints.push(PrimaryKey::new(
            format!("pk_{}", physical(&table.name, rename)),
            &borrowed,
        ));
    }
    for constraint in &table.constraints {
        let columns = quote_all(&constraint.columns, quote);
        let borrowed: Vec<&str> = columns.iter().map(String::as_str).collect();
        constraints.push(match constraint.kind.as_str() {
            "primaryKey" => PrimaryKey::new(&constraint.name, &borrowed),
            "unique" => UniqueConstraint::new(&constraint.name, &borrowed),
            "check" => CheckConstraint::new(
                &constraint.name,
                constraint
                    .expression
                    .clone()
                    .unwrap_or_else(|| panic!("{}: empty check", constraint.name)),
            ),
            "foreignKey" => {
                let references = constraint
                    .references
                    .as_deref()
                    .unwrap_or_else(|| panic!("{}: empty references", constraint.name));
                let referenced = quote_all(&constraint.references_columns, quote);
                let referenced_borrowed: Vec<&str> = referenced.iter().map(String::as_str).collect();
                ForeignKey::new(
                    &constraint.name,
                    &borrowed,
                    quote(&physical(references, rename)),
                    &referenced_borrowed,
                    None,
                    None,
                )
            }
            _ => panic!("{}: empty constraint", constraint.name),
        });
    }
    let built: Vec<Column> = specs
        .iter()
        .map(|column| {
            let mut built = Column::new(quote(&column.name), column.type_name.clone());
            if let Some(value) = &column.default {
                built = built.default(ColumnDefault::new(value.clone(), None));
            }
            if column.not_null {
                built = built.not_null(NotNull::new(None));
            }
            if column.primary_key && column.type_name == "int" {
                built = built.identity();
            }
            built
        })
        .collect();
    CreateTableStatement::new(quote(&physical(&table.name, rename)), &built, &constraints)
}

pub fn tables<Q>(
    name: &str,
    quote: Q,
    rename: Option<&HashMap<String, String>>,
) -> Vec<CreateTableStatement>
where
    Q: Fn(&str) -> String,
{
    let file = catalog();
    let schemas = file.schemas.get(name).unwrap_or_else(|| panic!("unknown schema {name}"));
    schemas
        .iter()
        .map(|table| statement_of(table, &quote, rename, &file.fragments))
        .collect()
}
