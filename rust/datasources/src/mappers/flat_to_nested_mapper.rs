use crate::mappers::i_flat_to_nested_mapper::FlatToNestedMapper as FlatToNestedMapperTrait;
use crate::mappers::mapped_value::{MappedRow, MappedValue};
use crate::mappers::type_map::TypeMap;
use crate::mappers::type_map_field::TypeMapField;
use crate::mappers::Mapper;
use crate::row::Row;
use crate::value::Value;
use std::collections::{HashMap, HashSet};

struct GroupPlan {
    key: String,
    fields: Vec<(String, String)>,
    dest_key: String,
}

pub struct FlatToNestedMapper {
    spec: TypeMap,
    scalars: Vec<(String, String)>,
    groups: Vec<(String, GroupPlan)>,
    parent_dest: String,
}

impl FlatToNestedMapper {
    pub fn new(spec: TypeMap) -> Self {
        let mut scalars = Vec::new();
        let mut groups = Vec::new();
        for (name, field) in &spec.fields {
            match field {
                TypeMapField::Column(source) => scalars.push((name.clone(), source.clone())),
                TypeMapField::Group(group) => {
                    let fields = string_fields(&group.fields);
                    let dest_key = dest_of(&fields, &group.key);
                    groups.push((
                        name.clone(),
                        GroupPlan {
                            key: group.key.clone(),
                            fields,
                            dest_key,
                        },
                    ));
                }
            }
        }
        let parent_dest = dest_of(&scalars, &spec.key);
        Self {
            spec,
            scalars,
            groups,
            parent_dest,
        }
    }

    fn cell(row: &Row, column: &str) -> Value {
        row.get(column).cloned().unwrap_or(Value::Null)
    }
}

fn string_fields(fields: &[(String, TypeMapField)]) -> Vec<(String, String)> {
    fields
        .iter()
        .filter_map(|(name, field)| match field {
            TypeMapField::Column(source) => Some((name.clone(), source.clone())),
            TypeMapField::Group(_) => None,
        })
        .collect()
}

fn dest_of(fields: &[(String, String)], column: &str) -> String {
    fields
        .iter()
        .find(|(_, source)| source == column)
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| column.to_string())
}

fn cell_key(value: Option<&Value>) -> Option<String> {
    match value {
        None | Some(Value::Null) => None,
        Some(value) => Some(value.wire_text()),
    }
}

fn mapped_cell_key(row: &MappedRow, name: &str) -> Option<String> {
    match row.get(name) {
        Some(MappedValue::Cell(value)) => cell_key(Some(value)),
        _ => None,
    }
}

fn passthrough(row: &Row) -> MappedRow {
    row.iter()
        .map(|(key, value)| (key.clone(), MappedValue::Cell(value.clone())))
        .collect()
}

impl Mapper<Vec<Row>, Vec<MappedRow>> for FlatToNestedMapper {
    fn map(&self, rows: Vec<Row>) -> Vec<MappedRow> {
        self.collapse(
            rows.into_iter()
                .map(|row| {
                    if self.groups.is_empty() {
                        passthrough(&row)
                    } else {
                        self.map_row(row)
                    }
                })
                .collect(),
        )
    }

    fn reverse_map(&self, values: Vec<MappedRow>) -> Vec<Row> {
        values.into_iter().flat_map(|value| self.explode(value)).collect()
    }
}

impl FlatToNestedMapper {
    fn map_row(&self, row: Row) -> MappedRow {
        let mut dest = MappedRow::new();
        for (name, column) in &self.scalars {
            dest.insert(name.clone(), MappedValue::Cell(Self::cell(&row, column)));
        }
        for (name, group) in &self.groups {
            dest.insert(
                name.clone(),
                MappedValue::Many(match cell_key(row.get(&group.key)) {
                    None => Vec::new(),
                    Some(_) => vec![{
                        let mut child = MappedRow::new();
                        for (field, column) in &group.fields {
                            child
                                .insert(field.clone(), MappedValue::Cell(Self::cell(&row, column)));
                        }
                        child
                    }],
                }),
            );
        }
        dest
    }

    fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow> {
        let mut by_parent: HashMap<String, MappedRow> = HashMap::new();
        let mut order = Vec::new();
        let mut seen = HashSet::new();
        for item in rows {
            let Some(parent) = mapped_cell_key(&item, &self.parent_dest) else {
                continue;
            };
            if !by_parent.contains_key(&parent) {
                self.mark(&mut seen, &parent, &item);
                order.push(parent.clone());
                by_parent.insert(parent, item);
                continue;
            }
            self.merge(
                by_parent.get_mut(&parent).expect("parent"),
                &item,
                &parent,
                &mut seen,
            );
        }
        order
            .into_iter()
            .filter_map(|key| by_parent.remove(&key))
            .collect()
    }

    fn explode(&self, value: MappedRow) -> Vec<Row> {
        let mut rows = vec![self.scalars_of(&value)];
        for (name, group) in &self.groups {
            let children = match value.get(name) {
                Some(MappedValue::Many(items)) if !items.is_empty() => items.clone(),
                _ => vec![MappedRow::new()],
            };
            rows = rows
                .into_iter()
                .flat_map(|row| {
                    children
                        .iter()
                        .map(|child| self.with_child(&row, group, child))
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        rows
    }

    fn scalars_of(&self, value: &MappedRow) -> Row {
        let mut row = Row::new();
        for (name, column) in &self.scalars {
            row.insert(
                column.clone(),
                match value.get(name) {
                    Some(MappedValue::Cell(cell)) => cell.clone(),
                    _ => Value::Null,
                },
            );
        }
        row
    }

    fn with_child(&self, row: &Row, group: &GroupPlan, child: &MappedRow) -> Row {
        let mut next = row.clone();
        for (field, column) in &group.fields {
            next.insert(
                column.clone(),
                match child.get(field) {
                    Some(MappedValue::Cell(cell)) => cell.clone(),
                    _ => Value::Null,
                },
            );
        }
        next
    }

    fn mark(&self, seen: &mut HashSet<String>, parent: &str, item: &MappedRow) {
        for (name, group) in &self.groups {
            let Some(MappedValue::Many(children)) = item.get(name) else {
                continue;
            };
            for child in children {
                if let Some(key) = mapped_cell_key(child, &group.dest_key) {
                    seen.insert(format!("{parent}:{name}:{key}"));
                }
            }
        }
    }

    fn merge(
        &self,
        into: &mut MappedRow,
        from: &MappedRow,
        parent: &str,
        seen: &mut HashSet<String>,
    ) {
        for (name, group) in &self.groups {
            let Some(MappedValue::Many(source)) = from.get(name) else {
                continue;
            };
            let Some(MappedValue::Many(dest)) = into.get_mut(name) else {
                continue;
            };
            for child in source {
                let Some(key) = mapped_cell_key(child, &group.dest_key) else {
                    continue;
                };
                let seen_key = format!("{parent}:{name}:{key}");
                if seen.contains(&seen_key) {
                    continue;
                }
                seen.insert(seen_key);
                dest.push(child.clone());
            }
        }
    }
}

impl FlatToNestedMapperTrait for FlatToNestedMapper {
    fn spec(&self) -> &TypeMap {
        &self.spec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mappers::type_map_group::TypeMapGroup;

    fn cell(pairs: &[(&str, Value)]) -> Row {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), value.clone()))
            .collect()
    }

    fn contacts_view() -> TypeMap {
        TypeMap::new(
            "id",
            vec![
                ("id".into(), TypeMapField::column("id")),
                ("first_name".into(), TypeMapField::column("first_name")),
                ("last_name".into(), TypeMapField::column("last_name")),
                ("email".into(), TypeMapField::column("email")),
                (
                    "contact_source_name".into(),
                    TypeMapField::column("contact_source_name"),
                ),
                (
                    "contact_type_name".into(),
                    TypeMapField::column("contact_type_name"),
                ),
                (
                    "addresses".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "address_id",
                        vec![
                            ("id".into(), TypeMapField::column("address_id")),
                            ("line1".into(), TypeMapField::column("address_line1")),
                            ("city".into(), TypeMapField::column("address_city")),
                            (
                                "address_type_name".into(),
                                TypeMapField::column("address_type_name"),
                            ),
                        ],
                    )),
                ),
                (
                    "phones".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "phone_id",
                        vec![
                            ("id".into(), TypeMapField::column("phone_id")),
                            ("number".into(), TypeMapField::column("phone_number")),
                            (
                                "phone_type_name".into(),
                                TypeMapField::column("phone_type_name"),
                            ),
                        ],
                    )),
                ),
            ],
        )
    }

    fn ada_home() -> Row {
        cell(&[
            ("id", Value::Int(7)),
            ("first_name", Value::from("Ada")),
            ("last_name", Value::from("Lovelace")),
            ("email", Value::from("ada@ex.com")),
            ("contact_source_name", Value::from("Web")),
            ("contact_type_name", Value::from("Person")),
            ("address_id", Value::Int(101)),
            ("address_line1", Value::from("10 Downing St")),
            ("address_city", Value::from("London")),
            ("address_type_name", Value::from("Home")),
            ("phone_id", Value::Int(201)),
            ("phone_number", Value::from("+44")),
            ("phone_type_name", Value::from("Mobile")),
        ])
    }

    fn ada_work() -> Row {
        let mut row = ada_home();
        row.insert("address_id".into(), Value::Int(102));
        row.insert("address_line1".into(), Value::from("1 Parliament Sq"));
        row
    }

    fn no_address() -> Row {
        cell(&[
            ("id", Value::Int(8)),
            ("first_name", Value::from("Charles")),
            ("last_name", Value::from("Babbage")),
            ("email", Value::Null),
            ("contact_source_name", Value::from("Import")),
            ("contact_type_name", Value::from("Person")),
            ("address_id", Value::Null),
            ("address_line1", Value::Null),
            ("address_city", Value::Null),
            ("address_type_name", Value::Null),
            ("phone_id", Value::Int(202)),
            ("phone_number", Value::from("555")),
            ("phone_type_name", Value::from("Work")),
        ])
    }

    fn many<'a>(row: &'a MappedRow, name: &str) -> &'a [MappedRow] {
        match row.get(name) {
            Some(MappedValue::Many(items)) => items,
            _ => &[],
        }
    }

    fn text(row: &MappedRow, name: &str) -> Value {
        match row.get(name) {
            Some(MappedValue::Cell(value)) => value.clone(),
            _ => Value::Null,
        }
    }

    #[test]
    fn maps_join_rows_to_a_nested_contact() {
        let mapper = FlatToNestedMapper::new(TypeMap::new(
            "id",
            vec![
                ("id".into(), TypeMapField::column("id")),
                ("first_name".into(), TypeMapField::column("first_name")),
                ("last_name".into(), TypeMapField::column("last_name")),
                ("email".into(), TypeMapField::column("email")),
                (
                    "contact_source_name".into(),
                    TypeMapField::column("contact_source_name"),
                ),
                (
                    "contact_type_name".into(),
                    TypeMapField::column("contact_type_name"),
                ),
                (
                    "addresses".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "address_id",
                        vec![
                            ("id".into(), TypeMapField::column("address_id")),
                            ("line1".into(), TypeMapField::column("address_line1")),
                            ("city".into(), TypeMapField::column("address_city")),
                            (
                                "address_type_name".into(),
                                TypeMapField::column("address_type_name"),
                            ),
                        ],
                    )),
                ),
                (
                    "phones".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "phone_id",
                        vec![
                            ("id".into(), TypeMapField::column("phone_id")),
                            ("number".into(), TypeMapField::column("phone_number")),
                            (
                                "phone_type_name".into(),
                                TypeMapField::column("phone_type_name"),
                            ),
                        ],
                    )),
                ),
            ],
        ));
        let ada_home = cell(&[
            ("id", Value::Int(7)),
            ("first_name", Value::from("Ada")),
            ("last_name", Value::from("Lovelace")),
            ("email", Value::from("ada@ex.com")),
            ("contact_source_name", Value::from("Web")),
            ("contact_type_name", Value::from("Person")),
            ("address_id", Value::Int(101)),
            ("address_line1", Value::from("10 Downing St")),
            ("address_city", Value::from("London")),
            ("address_type_name", Value::from("Home")),
            ("phone_id", Value::Int(201)),
            ("phone_number", Value::from("+44")),
            ("phone_type_name", Value::from("Mobile")),
        ]);
        let mut ada_work = ada_home.clone();
        ada_work.insert("address_id".into(), Value::Int(102));
        ada_work.insert("address_line1".into(), Value::from("1 Parliament Sq"));
        let nested = Mapper::map(&mapper, vec![ada_home.clone(), ada_work.clone()]);
        let flat = Mapper::reverse_map(&mapper, nested.clone());
        assert_eq!(many(&nested[0], "addresses").len(), 2);
        assert_eq!(many(&nested[0], "phones").len(), 1);
        assert_eq!(flat, vec![ada_home, ada_work]);
    }

    #[test]
    fn maps_one_denormalized_row_into_a_contact_with_0_1_children() {
        let mapper = FlatToNestedMapper::new(contacts_view());
        let mapped = Mapper::map(&mapper, vec![ada_home()]);
        let home = mapped[0].clone();
        assert_eq!(text(&home, "id"), Value::Int(7));
        assert_eq!(text(&home, "first_name"), Value::from("Ada"));
        assert_eq!(text(&home, "last_name"), Value::from("Lovelace"));
        assert_eq!(text(&home, "email"), Value::from("ada@ex.com"));
        assert_eq!(text(&home, "contact_source_name"), Value::from("Web"));
        assert_eq!(text(&home, "contact_type_name"), Value::from("Person"));
        assert_eq!(many(&home, "addresses").len(), 1);
        assert_eq!(text(&many(&home, "addresses")[0], "id"), Value::Int(101));
        assert_eq!(
            text(&many(&home, "addresses")[0], "line1"),
            Value::from("10 Downing St")
        );
        assert_eq!(
            text(&many(&home, "addresses")[0], "city"),
            Value::from("London")
        );
        assert_eq!(many(&home, "phones").len(), 1);
        assert_eq!(text(&many(&home, "phones")[0], "id"), Value::Int(201));
        assert_eq!(
            text(&many(&home, "phones")[0], "number"),
            Value::from("+44")
        );
        assert_eq!(
            text(&many(&home, "addresses")[0], "address_type_name"),
            Value::from("Home")
        );
        assert_eq!(
            text(&many(&home, "phones")[0], "phone_type_name"),
            Value::from("Mobile")
        );
        let missing = Mapper::map(&mapper, vec![no_address()])[0].clone();
        assert!(many(&missing, "addresses").is_empty());
        assert_eq!(many(&missing, "phones").len(), 1);
        assert_eq!(
            text(&many(&missing, "phones")[0], "number"),
            Value::from("555")
        );
    }

    #[test]
    fn reverse_maps_the_first_child_of_each_group() {
        let mapper = FlatToNestedMapper::new(contacts_view());
        assert_eq!(
            Mapper::reverse_map(&mapper, Mapper::map(&mapper, vec![ada_home()])),
            vec![ada_home()]
        );
        assert_eq!(
            Mapper::reverse_map(&mapper, Mapper::map(&mapper, vec![no_address()])),
            vec![no_address()]
        );
    }

    #[test]
    fn collapses_duplicate_join_rows_into_one_contact_with_unique_children() {
        let mapper = FlatToNestedMapper::new(contacts_view());
        let rows = Mapper::map(
            &mapper,
            vec![ada_home(), ada_work(), ada_home(), no_address()],
        );
        assert_eq!(rows.len(), 2);
        let ada = &rows[0];
        assert_eq!(text(ada, "id"), Value::Int(7));
        assert_eq!(many(ada, "addresses").len(), 2);
        assert_eq!(text(&many(ada, "addresses")[0], "id"), Value::Int(101));
        assert_eq!(text(&many(ada, "addresses")[1], "id"), Value::Int(102));
        assert_eq!(
            text(&many(ada, "addresses")[1], "line1"),
            Value::from("1 Parliament Sq")
        );
        assert_eq!(many(ada, "phones").len(), 1);
        let charles = &rows[1];
        assert_eq!(text(charles, "id"), Value::Int(8));
        assert!(many(charles, "addresses").is_empty());
        assert_eq!(many(charles, "phones").len(), 1);
    }

    #[test]
    fn collapses_already_mapped_rows() {
        let mapper = FlatToNestedMapper::new(contacts_view());
        let rows = mapper.collapse(vec![
            mapper.map_row(ada_home()),
            mapper.map_row(ada_work()),
        ]);
        assert_eq!(many(&rows[0], "addresses").len(), 2);
        assert_eq!(many(&rows[0], "phones").len(), 1);
    }

    #[test]
    fn skips_rows_without_a_parent_key_and_children_without_a_group_key() {
        let mapper = FlatToNestedMapper::new(contacts_view());
        let mut orphan = ada_home();
        orphan.insert("id".into(), Value::Null);
        let mut mapped = mapper.map_row(ada_home());
        mapped.insert(
            "addresses".into(),
            MappedValue::Many(vec![
                HashMap::from([
                    ("line1".into(), MappedValue::Cell(Value::from("x"))),
                    ("city".into(), MappedValue::Cell(Value::from("y"))),
                ]),
                HashMap::from([
                    ("id".into(), MappedValue::Cell(Value::Int(101))),
                    ("line1".into(), MappedValue::Cell(Value::from("dup"))),
                    ("city".into(), MappedValue::Cell(Value::from("London"))),
                ]),
            ]),
        );
        let mut extra = mapper.map_row(ada_work());
        extra.insert(
            "addresses".into(),
            MappedValue::Many(vec![HashMap::from([
                ("line1".into(), MappedValue::Cell(Value::from("ghost"))),
                ("city".into(), MappedValue::Cell(Value::from("None"))),
            ])]),
        );
        let rows = mapper.collapse(vec![mapper.map_row(orphan), mapped, extra]);
        let lines: Vec<_> = many(&rows[0], "addresses")
            .iter()
            .map(|address| text(address, "line1"))
            .collect();
        assert_eq!(lines, vec![Value::from("x"), Value::from("dup")]);
    }

    #[test]
    fn falls_back_to_the_spec_key_when_it_is_not_a_mapped_dest_field() {
        let mapper = FlatToNestedMapper::new(TypeMap::new(
            "id",
            vec![("name".into(), TypeMapField::column("name"))],
        ));
        let rows = Mapper::map(
            &mapper,
            vec![cell(&[("id", Value::Int(1)), ("name", Value::from("ada"))])],
        );
        assert_eq!(text(&rows[0], "id"), Value::Int(1));
        assert_eq!(text(&rows[0], "name"), Value::from("ada"));
    }

    #[test]
    fn omits_nested_group_fields_when_compiling_a_collection() {
        let mapper = FlatToNestedMapper::new(TypeMap::new(
            "id",
            vec![
                ("id".into(), TypeMapField::column("id")),
                (
                    "items".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "item_id",
                        vec![
                            ("id".into(), TypeMapField::column("item_id")),
                            ("title".into(), TypeMapField::column("item_title")),
                            (
                                "kids".into(),
                                TypeMapField::Group(TypeMapGroup::new(
                                    "kid_id",
                                    vec![("n".into(), TypeMapField::column("kid_n"))],
                                )),
                            ),
                        ],
                    )),
                ),
            ],
        ));
        let mapped = Mapper::map(
            &mapper,
            vec![cell(&[("id", Value::Int(1)), ("item_id", Value::Int(2))])],
        )[0]
            .clone();
        assert_eq!(text(&mapped, "id"), Value::Int(1));
        assert_eq!(many(&mapped, "items").len(), 1);
        assert_eq!(text(&many(&mapped, "items")[0], "id"), Value::Int(2));
        assert_eq!(text(&many(&mapped, "items")[0], "title"), Value::Null);
        assert!(!many(&mapped, "items")[0].contains_key("kids"));
        assert!(Mapper::map(&mapper, vec![Row::new()]).is_empty());
        let reversed = Mapper::reverse_map(&mapper, vec![MappedRow::new()]);
        assert_eq!(reversed[0].get("id"), Some(&Value::Null));
        assert_eq!(reversed[0].get("item_id"), Some(&Value::Null));
        assert_eq!(reversed[0].get("item_title"), Some(&Value::Null));
    }
}
