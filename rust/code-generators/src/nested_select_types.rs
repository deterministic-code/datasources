use crate::spec::{SpecType, SpecTypeField};
use datasources::mappers::{ChangeCasingMapper, Mapper, PluralizerMapper};
use datasources::{
    Casing, NestedSelectRelation, NestedSelectRelationKind, NestedSelectType,
};
use std::collections::{HashMap, HashSet};

pub fn from_spec(types: &[SpecType], root: &str) -> NestedSelectType {
    let tagged = types.iter().any(|item| item.tags.iter().any(|tag| tag == "datasource_type"));
    let catalog: HashMap<&str, &SpecType> = types
        .iter()
        .filter(|item| !tagged || item.tags.iter().any(|tag| tag == "datasource_type"))
        .map(|item| (item.name.as_str(), item))
        .collect();
    let name = if catalog.contains_key(root) {
        root
    } else {
        catalog
            .keys()
            .find(|item| entity(item) == root)
            .copied()
            .unwrap_or_else(|| panic!("Unknown nested select type \"{root}\""))
    };
    build(catalog[name], &catalog, &HashSet::new())
}

fn build(
    type_def: &SpecType,
    catalog: &HashMap<&str, &SpecType>,
    ancestors: &HashSet<String>,
) -> NestedSelectType {
    let mut next = ancestors.clone();
    next.insert(type_def.name.clone());
    let key = entity(
        type_def
            .fields
            .iter()
            .find(|field| field.is_id)
            .map(|field| field.name.as_str())
            .or_else(|| type_def.ids.as_ref().and_then(|ids| ids.first()).map(String::as_str))
            .unwrap_or("id"),
    );
    let properties = type_def
        .fields
        .iter()
        .filter(|field| field.references.is_none() && field.reference_pair.is_none())
        .map(|field| entity(&field.name))
        .collect();
    let objects = type_def
        .fields
        .iter()
        .filter_map(|field| {
            let (table, column) = target(field);
            if table.is_empty() || !catalog.contains_key(table.as_str()) || next.contains(&table) {
                return None;
            }
            Some(NestedSelectRelation::new(
                relation_name(&field.name, &table),
                NestedSelectRelationKind::Object,
                entity(&field.name),
                entity(&column),
                build(catalog[table.as_str()], catalog, &next),
            ))
        });
    let lists: Vec<_> = catalog.values().filter_map(|child| {
        if next.contains(&child.name) {
            return None;
        }
        child.fields.iter().find_map(|field| {
            let (table, column) = target(field);
            if table != type_def.name || column.is_empty() {
                return None;
            }
            Some(NestedSelectRelation::new(
                Mapper::map(&PluralizerMapper::new(), entity(&child.name)),
                NestedSelectRelationKind::List,
                key.clone(),
                entity(&field.name),
                build(child, catalog, &next),
            ))
        })
    }).collect();
    NestedSelectType::new(
        entity(&type_def.name),
        key,
        properties,
        objects.chain(lists).collect(),
    )
}

fn target(field: &SpecTypeField) -> (String, String) {
    if let Some(pair) = field.reference_pair.as_ref() {
        if pair.len() == 2 {
            return (pair[0].clone(), pair[1].clone());
        }
    }
    match field.references.as_deref().unwrap_or("").split_once('.') {
        Some((table, column)) => (table.to_string(), column.to_string()),
        None => (String::new(), String::new()),
    }
}

fn relation_name(field: &str, table: &str) -> String {
    let name = entity(field);
    if name.ends_with("Id") && name.len() > 2 {
        name[..name.len() - 2].to_string()
    } else {
        entity(table)
    }
}

fn entity(name: &str) -> String {
    Mapper::map(&ChangeCasingMapper::new(Casing::Snake, Casing::Pascal), name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::SpecType;

    fn field(name: &str, is_id: bool, references: Option<&str>) -> SpecTypeField {
        SpecTypeField {
            name: name.into(),
            type_name: "integer".into(),
            kind: "primitive".into(),
            base: "integer".into(),
            is_array: false,
            is_map: false,
            is_nullable: false,
            is_id,
            size: None,
            references: references.map(str::to_string),
            reference_pair: None,
            default_value: None,
            has_default: false,
            source: None,
            original: None,
        }
    }

    fn typed(name: &str, fields: Vec<SpecTypeField>) -> SpecType {
        SpecType {
            name: name.into(),
            tags: vec!["datasource_type".into()],
            kind: "shaped".into(),
            inherits: None,
            union: None,
            mapping: None,
            extract: None,
            remove_fields: None,
            ids: None,
            fields,
        }
    }

    #[test]
    fn builds_contacts_graph_from_spec() {
        let types = vec![
            typed(
                "contacts_base",
                vec![
                    field("id", true, None),
                    field("first_name", false, None),
                    field("contact_source_id", false, Some("contact_source.id")),
                ],
            ),
            typed(
                "contact_source",
                vec![field("id", true, None), field("name", false, None)],
            ),
            typed(
                "address",
                vec![
                    field("id", true, None),
                    field("contact_id", false, Some("contacts_base.id")),
                    field("line1", false, None),
                ],
            ),
        ];
        let graph = from_spec(&types, "contacts_base");
        assert_eq!(graph.name, "ContactsBase");
        assert_eq!(graph.key, "Id");
        assert!(graph.properties.iter().any(|name| name == "FirstName"));
        assert!(graph
            .relations
            .iter()
            .any(|item| item.name == "ContactSource" && item.kind == NestedSelectRelationKind::Object));
        assert!(graph
            .relations
            .iter()
            .any(|item| item.name == "Addresses" && item.kind == NestedSelectRelationKind::List));
    }
}
