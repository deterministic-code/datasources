use crate::builders::Join;
use crate::mappers::{TypeMap, TypeMapField, TypeMapGroup};
use super::nested_select_builder_options::NestedSelectBuilderOptions;
use super::nested_select_relation_kind::NestedSelectRelationKind;
use super::nested_select_statement::NestedSelectStatement;
use super::nested_select_type::NestedSelectType;
use super::select_field_aliases::select_field_aliases;
use crate::statements::SelectStatementOptions;
use crate::SelectStatement;
use std::collections::HashMap;

pub struct NestedSelectBuilder;

impl NestedSelectBuilder {
    pub fn build(
        root: &NestedSelectType,
        options: NestedSelectBuilderOptions<'_>,
    ) -> NestedSelectStatement {
        let mut fields = Vec::new();
        let mut joins = Vec::new();
        walk(
            root,
            &root.name,
            &root.name,
            "",
            &options.stop_at,
            false,
            &mut fields,
            &mut joins,
        );
        let aliases = select_field_aliases(&root.name, &fields, &[]);
        let by_path: HashMap<String, String> = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                (
                    field.clone(),
                    aliases[index].clone().unwrap_or_else(|| field.clone()),
                )
            })
            .collect();
        let field_refs: Vec<&str> = fields.iter().map(String::as_str).collect();
        NestedSelectStatement {
            select: SelectStatement::new(SelectStatementOptions {
                joins: &joins,
                where_clause: options.where_clause,
                ..SelectStatementOptions::new(&root.name, &field_refs)
            }),
            map: map_of(root, &root.name, "", &options.stop_at, &by_path),
        }
    }
}

fn walk(
    type_def: &NestedSelectType,
    field_prefix: &str,
    join_entity: &str,
    stop_prefix: &str,
    stop_at: &[String],
    outer: bool,
    fields: &mut Vec<String>,
    joins: &mut Vec<Join>,
) {
    for property in &type_def.properties {
        fields.push(format!("{field_prefix}.{property}"));
    }
    for relation in &type_def.relations {
        let path = if stop_prefix.is_empty() {
            relation.name.clone()
        } else {
            format!("{stop_prefix}.{}", relation.name)
        };
        if stopped(stop_at, &path) {
            continue;
        }
        let list = relation.kind == NestedSelectRelationKind::List;
        joins.push(if list || outer {
            Join::left(
                relation.target.name.clone(),
                format!("{join_entity}.{}", relation.from),
                format!("{}.{}", relation.target.name, relation.to),
            )
        } else {
            Join::inner(
                relation.target.name.clone(),
                format!("{join_entity}.{}", relation.from),
                format!("{}.{}", relation.target.name, relation.to),
            )
        });
        walk(
            &relation.target,
            &format!("{field_prefix}.{}", relation.target.name),
            &relation.target.name,
            &path,
            stop_at,
            outer || list,
            fields,
            joins,
        );
    }
}

fn map_of(
    type_def: &NestedSelectType,
    field_prefix: &str,
    stop_prefix: &str,
    stop_at: &[String],
    aliases: &HashMap<String, String>,
) -> TypeMap {
    TypeMap::new(
        aliases
            .get(&format!("{field_prefix}.{}", type_def.key))
            .cloned()
            .unwrap_or_else(|| type_def.key.clone()),
        fields_of(type_def, field_prefix, stop_prefix, stop_at, aliases),
    )
}

fn fields_of(
    type_def: &NestedSelectType,
    field_prefix: &str,
    stop_prefix: &str,
    stop_at: &[String],
    aliases: &HashMap<String, String>,
) -> Vec<(String, TypeMapField)> {
    let mut fields = Vec::new();
    for property in &type_def.properties {
        let alias = aliases
            .get(&format!("{field_prefix}.{property}"))
            .cloned()
            .unwrap_or_else(|| property.clone());
        fields.push((alias.clone(), TypeMapField::column(alias)));
    }
    for relation in &type_def.relations {
        let path = if stop_prefix.is_empty() {
            relation.name.clone()
        } else {
            format!("{stop_prefix}.{}", relation.name)
        };
        if stopped(stop_at, &path) {
            continue;
        }
        let child_prefix = format!("{field_prefix}.{}", relation.target.name);
        match relation.kind {
            NestedSelectRelationKind::List => {
                let key = aliases
                    .get(&format!("{child_prefix}.{}", relation.target.key))
                    .cloned()
                    .unwrap_or_else(|| relation.target.key.clone());
                fields.push((
                    relation.name.clone(),
                    TypeMapField::Group(TypeMapGroup::new(
                        key,
                        fields_of(&relation.target, &child_prefix, &path, stop_at, aliases),
                    )),
                ));
            }
            NestedSelectRelationKind::Object => {
                fields.extend(fields_of(
                    &relation.target,
                    &child_prefix,
                    &path,
                    stop_at,
                    aliases,
                ));
            }
        }
    }
    fields
}

fn stopped(stop_at: &[String], path: &str) -> bool {
    stop_at
        .iter()
        .any(|item| item == path || path.starts_with(&format!("{item}.")))
}
