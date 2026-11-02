use crate::converters::{
    is_column, peek_value, reflect, set_field, set_value, struct_fields, SchemaObject,
};
use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use facet::{Def, HasFields, Partial, Peek, Type, UserType};

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
        return map
            .iter()
            .map(|(key, value)| {
                let mut row = Row::new();
                if let Some(cell) = peek_value(key) {
                    row.insert("key".into(), cell);
                }
                if let Some(cell) = peek_value(value) {
                    row.insert("value".into(), cell);
                }
                row
            })
            .collect();
    }
    Vec::new()
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
        return map
            .iter()
            .map(|(key, value)| {
                let mut row = Row::new();
                if let Some(cell) = peek_value(key) {
                    row.insert("key".into(), cell);
                }
                if let Some(cell) = peek_value(value) {
                    row.insert("value".into(), cell);
                }
                row
            })
            .collect();
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
        Def::Map(_) => set_map(partial, rows),
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
                partial = match (is_column(field.shape()), row.get(field.name)) {
                    (true, Some(cell)) if *cell != Value::Null => {
                        set_field(partial, field.name, cell, field.shape(), true)?
                    }
                    _ => partial.set_nth_field_to_default(index).map_err(reflect)?,
                };
            }
        }
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn set_map<'a>(partial: Partial<'a>, rows: Vec<Row>) -> Result<Partial<'a>> {
    let mut partial = partial.init_map().map_err(reflect)?;
    for row in rows {
        let key = row
            .get("key")
            .or_else(|| row.get("Key"))
            .map(Value::wire_text)
            .unwrap_or_default();
        let value = row
            .get("value")
            .or_else(|| row.get("Value"))
            .map(Value::wire_text)
            .unwrap_or_default();
        partial = partial.set_key(key).map_err(reflect)?;
        partial = partial.set_value(value).map_err(reflect)?;
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
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct House {
        id: i64,
        kids: Vec<Kid>,
    }

    impl SchemaObject for Kid {}
    impl SchemaObject for House {}

    #[test]
    fn struct_payload_round_trips_list() {
        let house = House {
            id: 1,
            kids: vec![Kid {
                id: 2,
                label: "a".into(),
            }],
        };
        let rows = payload_from_struct(&house, "kids");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get("label"), Some(&Value::from("a")));
        let copied = set_payload_on_struct(&House::default(), "kids", rows).unwrap();
        assert_eq!(copied.kids[0].label, "a");
        assert_eq!(copied.kids[0].id, 2);
    }
}
