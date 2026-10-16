use super::contact_view::{AddressView, ContactView, PhoneView};
use datasources::{MappedRow, MappedValue, Value};

fn text(row: &MappedRow, key: &str) -> String {
    match row.get(key) {
        Some(MappedValue::Cell(Value::Null)) | None => String::new(),
        Some(MappedValue::Cell(value)) => value.wire_text(),
        _ => String::new(),
    }
}

fn id_of(row: &MappedRow, key: &str) -> i64 {
    match row.get(key) {
        Some(MappedValue::Cell(Value::Int(number))) => *number,
        Some(MappedValue::Cell(other)) => other.wire_text().parse().unwrap_or(0),
        _ => 0,
    }
}

fn many<'a>(row: &'a MappedRow, key: &str) -> &'a [MappedRow] {
    match row.get(key) {
        Some(MappedValue::Many(items)) => items,
        _ => &[],
    }
}

pub fn from_mapped_contacts(rows: &[MappedRow]) -> Vec<ContactView> {
    rows.iter()
        .map(|row| ContactView {
            id: id_of(row, "id"),
            first_name: text(row, "first_name"),
            last_name: text(row, "last_name"),
            email: text(row, "email"),
            contact_source_name: text(row, "contact_source_name"),
            contact_type_name: text(row, "contact_type_name"),
            addresses: many(row, "addresses")
                .iter()
                .map(|child| AddressView {
                    id: id_of(child, "id"),
                    line1: text(child, "line1"),
                    city: text(child, "city"),
                    address_type_name: text(child, "address_type_name"),
                })
                .collect(),
            phones: many(row, "phones")
                .iter()
                .map(|child| PhoneView {
                    id: id_of(child, "id"),
                    number: text(child, "number"),
                    phone_type_name: text(child, "phone_type_name"),
                })
                .collect(),
        })
        .collect()
}
