use std::collections::HashMap;
use std::fs;

use datasources::{
    CheckConstraint, Column, ColumnDefault, CreateTableStatement, ForeignKey, NotNull, PrimaryKey,
    ReferentialAction, TableConstraint, Text, UniqueConstraint,
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
    #[serde(rename = "referencesColumns")]
    references_columns: Option<Vec<String>>,
    #[serde(rename = "onDelete")]
    on_delete: Option<String>,
    #[serde(rename = "onUpdate")]
    on_update: Option<String>,
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

fn sqlite_type(canonical: &str) -> String {
    column_type("sqlite", canonical)
}

fn mysql_type(canonical: &str) -> String {
    column_type("mysql", canonical)
}

fn postgres_type(canonical: &str) -> String {
    column_type("postgres", canonical)
}

pub fn type_mapper(provider: &str) -> fn(&str) -> String {
    match provider {
        "sqlite" => sqlite_type,
        "mysql" => mysql_type,
        "postgres" => postgres_type,
        _ => panic!("unknown provider {provider}"),
    }
}

pub fn column_type(provider: &str, canonical: &str) -> String {
    match (provider, canonical) {
        ("sqlite", "int" | "bool") => "INTEGER",
        ("sqlite", "uuid" | "string" | "datetime") => "TEXT",
        ("sqlite", "decimal") => "NUMERIC",
        ("sqlite", "float") => "REAL",
        ("sqlite", "binary") => "BLOB",
        ("mysql", "int") => "INT",
        ("mysql", "uuid") => "CHAR(36)",
        ("mysql", "string" | "datetime") => "TEXT",
        ("mysql", "bool") => "TINYINT",
        ("mysql", "decimal") => "DECIMAL(18,4)",
        ("mysql", "float") => "DOUBLE",
        ("mysql", "binary") => "BLOB",
        ("postgres", "int") => "INTEGER",
        ("postgres", "uuid") => "UUID",
        ("postgres", "string") => "TEXT",
        ("postgres", "bool") => "BOOLEAN",
        ("postgres", "decimal") => "NUMERIC",
        ("postgres", "float") => "DOUBLE PRECISION",
        ("postgres", "datetime") => "TIMESTAMPTZ",
        ("postgres", "binary") => "BYTEA",
        ("sqlite" | "mysql" | "postgres", _) => panic!("Unknown {provider} type \"{canonical}\""),
        _ => panic!("unknown provider {provider}"),
    }
    .to_string()
}

fn action(name: Option<&str>) -> Option<ReferentialAction> {
    match name {
        None | Some("") => None,
        Some("cascade") => Some(ReferentialAction::Cascade),
        Some("setNull") => Some(ReferentialAction::SetNull),
        Some("setDefault") => Some(ReferentialAction::SetDefault),
        Some("restrict") => Some(ReferentialAction::Restrict),
        Some("noAction") => Some(ReferentialAction::NoAction),
        Some(other) => panic!("unknown referential action {other}"),
    }
}

fn columns_of(table: &TableSpec, fragments: &HashMap<String, Vec<ColumnSpec>>) -> Vec<ColumnSpec> {
    let mut columns = Vec::new();
    for fragment in &table.include {
        columns.extend(
            fragments
                .get(fragment)
                .unwrap_or_else(|| panic!("unknown fragment {fragment}"))
                .clone(),
        );
    }
    columns.extend(table.columns.clone());
    columns
}

fn quote_all<Q: Fn(&str) -> String + ?Sized>(names: &[String], quote: &Q) -> Vec<String> {
    names.iter().map(|name| quote(name)).collect()
}

fn constraint_of<Q: Fn(&str) -> String + ?Sized>(
    constraint: &ConstraintSpec,
    quote: &Q,
    rename: Option<&HashMap<String, String>>,
) -> TableConstraint {
    let columns = quote_all(&constraint.columns, quote);
    let borrowed: Vec<&str> = columns.iter().map(String::as_str).collect();
    match constraint.kind.as_str() {
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
            let referenced = quote_all(constraint.references_columns.as_deref().unwrap_or(&[]), quote);
            let referenced_borrowed: Vec<&str> = referenced.iter().map(String::as_str).collect();
            ForeignKey::new(
                &constraint.name,
                &borrowed,
                quote(&physical(references, rename)),
                &referenced_borrowed,
                action(constraint.on_delete.as_deref()),
                action(constraint.on_update.as_deref()),
            )
        }
        _ => panic!("{}: empty constraint", constraint.name),
    }
}

fn column_of<Q, M>(
    column: &ColumnSpec,
    provider: &str,
    quote: &Q,
    map_type: &M,
    executable_text: bool,
    inline: bool,
) -> Column
where
    Q: Fn(&str) -> String + ?Sized,
    M: Fn(&str) -> String + ?Sized,
{
    let mapped = if executable_text || provider == "sqlite" {
        map_type(&column.type_name)
    } else {
        column.type_name.clone()
    };
    let type_name = if inline && column.primary_key {
        format!(
            "{mapped}{}",
            match provider {
                "mysql" => " PRIMARY KEY AUTO_INCREMENT",
                "postgres" => " GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY",
                _ => " PRIMARY KEY",
            }
        )
    } else {
        mapped
    };
    let mut built = Column::new(quote(&column.name), type_name);
    if let Some(value) = &column.default {
        built = built.default(ColumnDefault::new(value.clone(), None));
    }
    if column.not_null {
        built = built.not_null(NotNull::new(None));
    }
    if !executable_text && provider != "sqlite" && column.primary_key && column.type_name == "int" {
        built = built.identity();
    }
    built
}

fn statement_of<Q, M>(
    table: &TableSpec,
    provider: &str,
    quote: &Q,
    map_type: &M,
    rename: Option<&HashMap<String, String>>,
    executable_text: bool,
    fragments: &HashMap<String, Vec<ColumnSpec>>,
) -> CreateTableStatement
where
    Q: Fn(&str) -> String + ?Sized,
    M: Fn(&str) -> String + ?Sized,
{
    if !matches!(provider, "sqlite" | "mysql" | "postgres") {
        panic!("unknown provider {provider}");
    }
    let columns = columns_of(table, fragments);
    let keys: Vec<&str> = columns
        .iter()
        .filter(|column| column.primary_key)
        .map(|column| column.name.as_str())
        .collect();
    let inline = !keys.is_empty()
        && (provider == "sqlite"
            || (executable_text
                && keys.iter().all(|key| {
                    columns
                        .iter()
                        .find(|column| column.name == *key)
                        .is_some_and(|column| column.type_name == "int")
                })));
    let mut constraints = Vec::new();
    if !keys.is_empty() && !inline {
        let quoted = quote_all(&keys.iter().map(|key| (*key).to_string()).collect::<Vec<_>>(), quote);
        let borrowed: Vec<&str> = quoted.iter().map(String::as_str).collect();
        constraints.push(PrimaryKey::new(
            format!("pk_{}", physical(&table.name, rename)),
            &borrowed,
        ));
    }
    for constraint in &table.constraints {
        constraints.push(constraint_of(constraint, quote, rename));
    }
    let built: Vec<Column> = columns
        .iter()
        .map(|column| column_of(column, provider, quote, map_type, executable_text, inline))
        .collect();
    CreateTableStatement::new(quote(&physical(&table.name, rename)), &built, &constraints)
}

pub fn tables<Q, M>(
    name: &str,
    provider: &str,
    quote: Q,
    map_type: M,
    rename: Option<&HashMap<String, String>>,
    executable_text: bool,
) -> Vec<CreateTableStatement>
where
    Q: Fn(&str) -> String,
    M: Fn(&str) -> String,
{
    let file = catalog();
    let schemas = file.schemas.get(name).unwrap_or_else(|| panic!("unknown schema {name}"));
    schemas
        .iter()
        .map(|table| statement_of(table, provider, &quote, &map_type, rename, executable_text, &file.fragments))
        .collect()
}

pub fn schema_sql<Q, M>(
    name: &str,
    provider: &str,
    quote: Q,
    map_type: M,
    rename: Option<&HashMap<String, String>>,
) -> String
where
    Q: Fn(&str) -> String,
    M: Fn(&str) -> String,
{
    tables(name, provider, quote, map_type, rename, true)
        .iter()
        .map(Text::get_text)
        .collect::<Vec<_>>()
        .join(";\n")
}
