use crate::contact_view::{AddressView, ContactView, PhoneView};
use datasources::{Row, Value};

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        None | Some(Value::Null) => String::new(),
        Some(value) => value.wire_text(),
    }
}

fn id_of(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(number)) => *number,
        Some(other) => other.wire_text().parse().unwrap_or(0),
        None => 0,
    }
}

fn parse_rows(encoded: &str) -> Vec<Row> {
    if encoded.is_empty() {
        return Vec::new();
    }
    encoded
        .split('|')
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.split(',')
                .filter_map(|pair| {
                    let (key, value) = pair.split_once('=')?;
                    Some((key.to_string(), Value::from(value)))
                })
                .collect()
        })
        .collect()
}

pub fn from_enriched_contacts(rows: &[Row]) -> Vec<ContactView> {
    rows.iter()
        .map(|row| ContactView {
            id: id_of(row, "id"),
            first_name: text(row, "first_name"),
            last_name: text(row, "last_name"),
            email: text(row, "email"),
            contact_source_name: text(row, "contact_source_name"),
            contact_type_name: text(row, "contact_type_name"),
            addresses: parse_rows(&text(row, "addresses"))
                .into_iter()
                .map(|child| AddressView {
                    id: id_of(&child, "id"),
                    line1: text(&child, "line1"),
                    city: text(&child, "city"),
                })
                .collect(),
            phones: parse_rows(&text(row, "phones"))
                .into_iter()
                .map(|child| PhoneView {
                    id: id_of(&child, "id"),
                    number: text(&child, "number"),
                    label: text(&child, "label"),
                })
                .collect(),
        })
        .collect()
}
