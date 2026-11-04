use crate::converters::{
    is_column, peek_value, reflect, set_changed_field, set_field, set_value, struct_fields,
    SchemaObject,
};
use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use facet::{Def, Field, HasFields, Partial, Peek, Type, UserType};

pub fn payload_from_row(row: &Row, name: &str) -> Vec<Row> {
    decode_rows(row.get(name))
}

pub fn set_payload_on_row(row: &mut Row, name: &str, rows: Vec<Row>) {
    row.insert(name.to_string(), encode_rows(&rows));
}

pub fn payload_from_struct<T: SchemaObject>(item: &T, name: &str) -> Vec<Row> {
    let Ok(st) = Peek::new(item).into_struct() else {
        return Vec::new();
    };
    let Ok(field) = st.field_by_name(name) else {
        return Vec::new();
    };
    if let Ok(list) = field.into_list() {
        return list.iter().filter_map(row_from_peek).collect();
    }
    if let Ok(map) = field.into_map() {
        return map.iter().map(|(key, value)| row_from_map_entry(key, value)).collect();
    }
    Vec::new()
}

pub fn cell_from_row(row: &Row, name: &str) -> Option<Value> {
    row.get(name).cloned()
}

pub fn set_cell_on_row(row: &mut Row, name: &str, value: Value) {
    row.insert(name.to_string(), value);
}

pub fn cell_from_struct<T: SchemaObject>(item: &T, name: &str) -> Option<Value> {
    let st = Peek::new(item).into_struct().ok()?;
    peek_value(st.field_by_name(name).ok()?)
}

pub fn set_cell_on_struct<T: SchemaObject>(item: &T, name: &str, cell: Value) -> Result<T> {
    let fields = struct_fields(T::SHAPE)?;
    let Some(field) = fields.iter().find(|field| field.name == name) else {
        return Ok(item.clone());
    };
    if !is_column(field.shape()) || cell == Value::Null {
        return Ok(item.clone());
    }
    let partial = Partial::alloc::<T>().map_err(reflect)?.set(item.clone()).map_err(reflect)?;
    let partial = set_changed_field(partial, name, &cell, field.shape())?;
    partial.build().map_err(reflect)?.materialize().map_err(reflect)
}

pub fn typed_list_from_struct<T: SchemaObject, R: SchemaObject>(item: &T, name: &str) -> Vec<R> {
    let Ok(st) = Peek::new(item).into_struct() else {
        return Vec::new();
    };
    let Ok(field) = st.field_by_name(name) else {
        return Vec::new();
    };
    let Ok(list) = field.into_list() else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|value| value.get::<R>().ok().cloned())
        .collect()
}

pub fn set_typed_list_on_struct<T: SchemaObject, R: SchemaObject>(
    item: &T,
    name: &str,
    items: Vec<R>,
) -> Result<T> {
    let st = Peek::new(item).into_struct().map_err(reflect)?;
    let fields = struct_fields(T::SHAPE)?;
    let mut partial = Partial::alloc::<T>().map_err(reflect)?;
    for (index, field) in fields.iter().enumerate() {
        partial = if field.name == name {
            let mut partial = partial
                .begin_field(field.name)
                .map_err(reflect)?
                .init_list_with_capacity(items.len())
                .map_err(reflect)?;
            for child in &items {
                partial = partial.begin_list_item().map_err(reflect)?;
                partial = partial.set(child.clone()).map_err(reflect)?;
                partial = partial.end().map_err(reflect)?;
            }
            partial.end().map_err(reflect)?
        } else {
            copy_field(partial, index, field.shape(), st.field(index).map_err(reflect)?)?
        };
    }
    partial.build().map_err(reflect)?.materialize().map_err(reflect)
}

pub fn set_one_on_struct<T: SchemaObject, R: SchemaObject>(item: &T, name: &str, related: R) -> Result<T> {
    let st = Peek::new(item).into_struct().map_err(reflect)?;
    let fields = struct_fields(T::SHAPE)?;
    let mut partial = Partial::alloc::<T>().map_err(reflect)?;
    for (index, field) in fields.iter().enumerate() {
        partial = if field.name == name {
            partial
                .begin_field(field.name)
                .map_err(reflect)?
                .set(related.clone())
                .map_err(reflect)?
                .end()
                .map_err(reflect)?
        } else {
            copy_field(partial, index, field.shape(), st.field(index).map_err(reflect)?)?
        };
    }
    partial.build().map_err(reflect)?.materialize().map_err(reflect)
}

pub fn set_payload_on_struct<T: SchemaObject>(item: &T, name: &str, rows: Vec<Row>) -> Result<T> {
    let st = Peek::new(item).into_struct().map_err(reflect)?;
    let fields = struct_fields(T::SHAPE)?;
    let mut partial = Partial::alloc::<T>().map_err(reflect)?;
    for (index, field) in fields.iter().enumerate() {
        partial = if field.name == name {
            let partial = partial.begin_field(field.name).map_err(reflect)?;
            set_destination(partial, field.shape(), rows.clone())?.end().map_err(reflect)?
        } else {
            copy_field(partial, index, field.shape(), st.field(index).map_err(reflect)?)?
        };
    }
    partial.build().map_err(reflect)?.materialize().map_err(reflect)
}

fn row_from_map_entry(key: Peek<'_, '_>, value: Peek<'_, '_>) -> Row {
    let mut row = Row::new();
    if let Some(cell) = peek_value(key) {
        row.insert("key".into(), cell);
    }
    if let Some(cell) = peek_value(value) {
        row.insert("value".into(), cell);
    } else if let Some(fields) = row_from_peek(value) {
        for (name, cell) in fields {
            row.insert(name, cell);
        }
    }
    row
}

fn row_from_peek(value: Peek<'_, '_>) -> Option<Row> {
    let st = value.into_struct().ok()?;
    let mut row = Row::new();
    for (field, cell) in st.fields() {
        if !is_column(field.shape()) {
            continue;
        }
        if let Some(value) = peek_value(cell) {
            row.insert(field.name.to_string(), value);
        }
    }
    Some(row)
}

fn copy_field<'a>(
    partial: Partial<'a>,
    index: usize,
    shape: &'static facet::Shape,
    value: Peek<'_, '_>,
) -> Result<Partial<'a>> {
    let partial = partial.begin_nth_field(index).map_err(reflect)?;
    let partial = if is_column(shape) {
        match peek_value(value) {
            Some(cell) if cell != Value::Null => set_value(partial, &cell, shape, true)?,
            _ if shape.type_identifier.contains("DateTime") => {
                set_value(partial, &Value::DateTime(chrono::DateTime::<chrono::Utc>::default()), shape, true)?
            }
            _ => partial.set_default().map_err(reflect)?,
        }
    } else {
        set_destination(partial, shape, payload_from_peek(value))?
    };
    partial.end().map_err(reflect)
}

fn payload_from_peek(value: Peek<'_, '_>) -> Vec<Row> {
    if let Ok(list) = value.into_list() {
        return list.iter().filter_map(row_from_peek).collect();
    }
    if let Ok(map) = value.into_map() {
        return map.iter().map(|(key, value)| row_from_map_entry(key, value)).collect();
    }
    Vec::new()
}

fn set_destination<'a>(
    partial: Partial<'a>,
    shape: &'static facet::Shape,
    rows: Vec<Row>,
) -> Result<Partial<'a>> {
    match shape.def {
        Def::List(list) => set_list(partial, list.t(), rows),
        Def::Map(map) => set_map(partial, map.v(), rows),
        _ => Ok(partial.set_default().map_err(reflect)?),
    }
}

fn set_list<'a>(
    partial: Partial<'a>,
    child: &'static facet::Shape,
    rows: Vec<Row>,
) -> Result<Partial<'a>> {
    let mut partial = partial.init_list_with_capacity(rows.len()).map_err(reflect)?;
    let fields = match child.ty {
        Type::User(UserType::Struct(_)) => struct_fields(child)?,
        _ => &[],
    };
    for row in rows {
        partial = partial.begin_list_item().map_err(reflect)?;
        if fields.is_empty() {
            if let Some(Value::Text(text)) = row.values().next() {
                partial = partial.set(text.clone()).map_err(reflect)?;
            } else {
                partial = partial.set_default().map_err(reflect)?;
            }
        } else {
            for (index, field) in fields.iter().enumerate() {
                partial = set_row_field(partial, index, field, row.get(field.name))?;
            }
        }
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn set_map<'a>(
    partial: Partial<'a>,
    value: &'static facet::Shape,
    rows: Vec<Row>,
) -> Result<Partial<'a>> {
    let mut partial = partial.init_map().map_err(reflect)?;
    for row in rows {
        let key = row
            .get("key")
            .or_else(|| row.get("Key"))
            .map(Value::wire_text)
            .unwrap_or_default();
        partial = partial.set_key(key).map_err(reflect)?;
        partial = if is_column(value) {
            let cell = row
                .get("value")
                .or_else(|| row.get("Value"))
                .map(Value::wire_text)
                .unwrap_or_default();
            partial.set_value(cell).map_err(reflect)?
        } else {
            set_map_value(partial.begin_value().map_err(reflect)?, value, &row)?
                .end()
                .map_err(reflect)?
        };
    }
    Ok(partial)
}

fn set_row_field<'a>(
    partial: Partial<'a>,
    index: usize,
    field: &Field,
    cell: Option<&Value>,
) -> Result<Partial<'a>> {
    match (is_column(field.shape()), cell) {
        (true, Some(cell)) if *cell != Value::Null => {
            set_field(partial, field.name, cell, field.shape(), true)
        }
        (true, _) if field.shape().type_identifier.contains("DateTime") => set_field(
            partial,
            field.name,
            &Value::DateTime(chrono::DateTime::<chrono::Utc>::default()),
            field.shape(),
            true,
        ),
        _ => partial.set_nth_field_to_default(index).map_err(reflect),
    }
}

fn set_map_value<'a>(
    mut partial: Partial<'a>,
    value: &'static facet::Shape,
    row: &Row,
) -> Result<Partial<'a>> {
    for (index, field) in struct_fields(value)?.iter().enumerate() {
        partial = set_row_field(partial, index, field, row.get(field.name))?;
    }
    Ok(partial)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converters::SchemaObject;
    use facet::Facet;

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct Kid {
        id: i64,
        label: String,
        extra: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct Pref {
        locale: String,
        timezone: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct House {
        id: i64,
        kids: Vec<Kid>,
        settings: std::collections::HashMap<String, String>,
        prefs: std::collections::HashMap<String, Pref>,
    }

    impl SchemaObject for Kid {}
    impl SchemaObject for Pref {}
    impl SchemaObject for House {}

    #[test]
    fn struct_payload_round_trips_list() {
        let house = House {
            id: 1,
            kids: vec![Kid {
                id: 2,
                label: "a".into(),
                extra: None,
            }],
            ..House::default()
        };
        let rows = payload_from_struct(&house, "kids");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get("label"), Some(&Value::from("a")));
        let copied = set_payload_on_struct(&House::default(), "kids", rows).unwrap();
        assert_eq!(copied.kids[0].label, "a");
        assert_eq!(copied.kids[0].id, 2);
    }

    #[test]
    fn set_related_keeps_option_list_fields() {
        use crate::entity::{assign_related, Entity};

        let house = House {
            kids: vec![Kid {
                id: 2,
                label: "home".into(),
                extra: Some("Room 12".into()),
            }],
            ..House::default()
        };
        let collected = house.related::<Kid>("kids");
        assert_eq!(collected[0].extra.as_deref(), Some("Room 12"));
        let mut dest = House::default();
        assign_related(&mut dest, "kids", collected).unwrap();
        assert_eq!(dest.kids[0].extra.as_deref(), Some("Room 12"));
        let enrichment = crate::Enrichment::new(
            "kids",
            "id",
            "house_id",
            None,
            crate::EnrichmentPropertyType::List,
        );
        let from_children = crate::create_enrichment_strategy::children::<House, Kid>(&enrichment, &house)
            .unwrap();
        assert_eq!(from_children[0].extra.as_deref(), Some("Room 12"));
        dest
            .set_related(
                "kids",
                vec![Kid {
                    id: 2,
                    label: "home".into(),
                    extra: Some("Room 12".into()),
                }],
            )
            .unwrap();
        assert_eq!(dest.kids[0].extra.as_deref(), Some("Room 12"));
    }

    #[test]
    fn struct_payload_round_trips_string_map() {
        let house = House {
            settings: std::collections::HashMap::from([("theme".into(), "dark".into())]),
            ..House::default()
        };
        let rows = payload_from_struct(&house, "settings");
        assert_eq!(rows[0].get("key"), Some(&Value::from("theme")));
        assert_eq!(rows[0].get("value"), Some(&Value::from("dark")));
        let copied = set_payload_on_struct(&House::default(), "settings", rows).unwrap();
        assert_eq!(copied.settings.get("theme"), Some(&"dark".to_string()));
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct SettingRow {
        id: i32,
        uuid: String,
        created: chrono::DateTime<chrono::Utc>,
        updated: chrono::DateTime<chrono::Utc>,
        version: Vec<u8>,
        contact_id: i32,
        key: String,
        value: String,
    }

    impl SchemaObject for SettingRow {}

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct PrefRow {
        id: i32,
        key: String,
        locale: String,
        timezone: String,
    }

    impl SchemaObject for PrefRow {}

    #[test]
    fn dictionary_rows_convert_to_child() {
        let house = House {
            settings: std::collections::HashMap::from([("theme".into(), "dark".into())]),
            prefs: std::collections::HashMap::from([(
                "home".into(),
                Pref {
                    locale: "en-GB".into(),
                    timezone: "Europe/London".into(),
                },
            )]),
            ..House::default()
        };
        let settings = crate::create_enrichment_strategy::children::<House, SettingRow>(
            &crate::Enrichment::new(
                "settings",
                "id",
                "contact_id",
                Some("value".into()),
                crate::EnrichmentPropertyType::Dictionary,
            ),
            &house,
        )
        .unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].key, "theme");
        assert_eq!(settings[0].value, "dark");
        let prefs = crate::create_enrichment_strategy::children::<House, Pref>(
            &crate::Enrichment::new(
                "prefs",
                "id",
                "contact_id",
                None,
                crate::EnrichmentPropertyType::Dictionary,
            ),
            &house,
        )
        .unwrap();
        assert_eq!(prefs.len(), 1);
        assert_eq!(prefs[0].locale, "en-GB");
        let prefs_rows = crate::create_enrichment_strategy::children::<House, PrefRow>(
            &crate::Enrichment::new(
                "prefs",
                "id",
                "contact_id",
                None,
                crate::EnrichmentPropertyType::Dictionary,
            ),
            &house,
        )
        .unwrap();
        assert_eq!(prefs_rows[0].key, "home");
        assert_eq!(prefs_rows[0].locale, "en-GB");
        assert_eq!(prefs_rows[0].id, 0);
        let stamped = set_cell_on_struct(&settings[0], "contact_id", Value::Int(1)).unwrap();
        assert_eq!(stamped.key, "theme");
        assert_eq!(stamped.contact_id, 1);
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct NamedSettingRow {
        id: i32,
        contact_id: i32,
        name: String,
        value: String,
    }

    impl SchemaObject for NamedSettingRow {}

    #[test]
    fn dictionary_rows_use_name_when_child_has_no_key() {
        let house = House {
            settings: std::collections::HashMap::from([("theme".into(), "dark".into())]),
            ..House::default()
        };
        let settings = crate::create_enrichment_strategy::children::<House, NamedSettingRow>(
            &crate::Enrichment::new(
                "settings",
                "id",
                "contact_id",
                Some("value".into()),
                crate::EnrichmentPropertyType::Dictionary,
            ),
            &house,
        )
        .unwrap();
        assert_eq!(settings[0].name, "theme");
        assert_eq!(settings[0].value, "dark");
    }

    #[test]
    fn set_cell_keeps_dictionary_maps() {
        let house = House {
            settings: std::collections::HashMap::from([("theme".into(), "dark".into())]),
            prefs: std::collections::HashMap::from([(
                "home".into(),
                Pref {
                    locale: "en-GB".into(),
                    timezone: "Europe/London".into(),
                },
            )]),
            ..House::default()
        };
        let copied = set_cell_on_struct(&house, "id", Value::Int(9)).unwrap();
        assert_eq!(copied.settings.get("theme"), Some(&"dark".to_string()));
        assert_eq!(copied.prefs["home"].locale, "en-GB");
    }

    #[test]
    fn struct_payload_round_trips_object_map() {
        let house = House {
            prefs: std::collections::HashMap::from([(
                "home".into(),
                Pref {
                    locale: "en-GB".into(),
                    timezone: "Europe/London".into(),
                },
            )]),
            ..House::default()
        };
        let rows = payload_from_struct(&house, "prefs");
        assert_eq!(rows[0].get("key"), Some(&Value::from("home")));
        assert_eq!(rows[0].get("locale"), Some(&Value::from("en-GB")));
        assert_eq!(rows[0].get("value"), None);
        let copied = set_payload_on_struct(&House::default(), "prefs", rows).unwrap();
        assert_eq!(copied.prefs["home"].locale, "en-GB");
        assert_eq!(copied.prefs["home"].timezone, "Europe/London");
    }

    #[derive(Clone, Debug, PartialEq, Facet)]
    struct AddressStamp {
        id: i32,
        contact_id: String,
        line2: Option<String>,
        created: chrono::DateTime<chrono::Utc>,
        latitude: Option<i32>,
        address_type_name: String,
    }

    impl Default for AddressStamp {
        fn default() -> Self {
            Self {
                id: 0,
                contact_id: String::new(),
                line2: None,
                created: chrono::DateTime::<chrono::Utc>::default(),
                latitude: None,
                address_type_name: String::new(),
            }
        }
    }

    impl SchemaObject for AddressStamp {}

    #[test]
    fn stamp_contact_id_keeps_option_line2() {
        use crate::create_enrichment_strategy::set;

        let mut address = AddressStamp {
            line2: Some("Room 12".into()),
            address_type_name: "Home".into(),
            created: chrono::DateTime::parse_from_rfc3339("2024-06-01T12:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            ..AddressStamp::default()
        };
        set(&mut address, "contact_id", Value::Int(9)).unwrap();
        assert_eq!(address.contact_id, "9");
        assert_eq!(address.line2.as_deref(), Some("Room 12"));
        assert_eq!(address.address_type_name, "Home");
        assert!(address.latitude.is_none());
    }
}
