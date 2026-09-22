use crate::group_view::GroupView;
use crate::role_view::RoleView;
use crate::user_view::UserView;
use datasources::{MappedRow, MappedValue, Value};

fn cell<'a>(row: &'a MappedRow, name: &str) -> Option<&'a Value> {
    match row.get(name) {
        Some(MappedValue::Cell(value)) => Some(value),
        _ => None,
    }
}

fn many(row: &MappedRow, name: &str) -> Vec<MappedRow> {
    match row.get(name) {
        Some(MappedValue::Many(items)) => items.clone(),
        _ => vec![],
    }
}

fn id(row: &MappedRow, name: &str) -> i64 {
    match cell(row, name) {
        Some(Value::Int(number)) => *number,
        Some(Value::Text(text)) => text.parse().unwrap_or(0),
        Some(Value::Float(number)) => *number as i64,
        _ => 0,
    }
}

fn text(row: &MappedRow, name: &str) -> String {
    match cell(row, name) {
        Some(Value::Text(value)) => value.clone(),
        Some(Value::DateTime(date)) => Value::iso_datetime(*date),
        Some(Value::Int(number)) => number.to_string(),
        Some(Value::Null) | None => String::new(),
        Some(value) => value.wire_text(),
    }
}

fn flag(row: &MappedRow, name: &str) -> bool {
    match cell(row, name) {
        Some(Value::Bool(value)) => *value,
        Some(Value::Int(number)) => *number != 0,
        _ => false,
    }
}

pub fn from_mapped(rows: &[MappedRow]) -> Vec<UserView> {
    let mut users: Vec<UserView> = rows
        .iter()
        .map(|row| {
            let mut roles: Vec<RoleView> = many(row, "roles")
                .iter()
                .map(|role| RoleView {
                    id: id(role, "id"),
                    name: text(role, "name"),
                    is_enabled: flag(role, "is_enabled"),
                })
                .collect();
            roles.sort_by_key(|role| role.id);
            let mut groups: Vec<GroupView> = many(row, "groups")
                .iter()
                .map(|group| GroupView {
                    id: id(group, "id"),
                    name: text(group, "name"),
                    is_enabled: flag(group, "is_enabled"),
                })
                .collect();
            groups.sort_by_key(|group| group.id);
            UserView {
                id: id(row, "id"),
                username: text(row, "username"),
                is_enabled: flag(row, "is_enabled"),
                created: text(row, "created"),
                roles,
                groups,
            }
        })
        .collect();
    users.sort_by_key(|user| user.id);
    users
}
