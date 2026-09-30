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

fn nested_text(row: &Row, key: &str, field: &str) -> String {
    let raw = text(row, key);
    let prefix = format!("{field}=");
    raw.split(',')
        .find_map(|pair| pair.strip_prefix(&prefix))
        .unwrap_or("")
        .replace("%7C", "|")
        .replace("%2C", ",")
        .replace("%3D", "=")
        .replace("%25", "%")
}

fn unescape(text: &str) -> String {
    text.replace("%7C", "|")
        .replace("%2C", ",")
        .replace("%3D", "=")
        .replace("%25", "%")
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
                    Some((key.to_string(), Value::from(unescape(value))))
                })
                .collect()
        })
        .collect()
}

pub fn from_enriched_contacts(rows: &[Row]) -> Vec<ContactView> {
    rows.iter()
        .map(|row| ContactView {
            id: id_of(row, "Id"),
            first_name: text(row, "FirstName"),
            last_name: text(row, "LastName"),
            email: text(row, "Email"),
            contact_source_name: nested_text(row, "ContactSource", "Name"),
            contact_type_name: nested_text(row, "ContactType", "Name"),
            addresses: parse_rows(&text(row, "Addresses"))
                .into_iter()
                .map(|child| AddressView {
                    id: id_of(&child, "Id"),
                    line1: text(&child, "Line1"),
                    city: text(&child, "City"),
                    address_type_name: nested_text(&child, "AddressType", "Name"),
                })
                .collect(),
            phones: parse_rows(&text(row, "Phones"))
                .into_iter()
                .map(|child| PhoneView {
                    id: id_of(&child, "Id"),
                    number: text(&child, "Number"),
                    phone_type_name: nested_text(&child, "PhoneType", "Name"),
                })
                .collect(),
        })
        .collect()
}
