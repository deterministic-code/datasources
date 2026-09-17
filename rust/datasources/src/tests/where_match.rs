use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;
use std::collections::HashMap;

pub fn row_matches(row: &Row, predicate: &dyn Where) -> bool {
    if predicate.get_text() == "1 = 0" {
        return false;
    }
    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
    for parameter in predicate.parameters() {
        groups
            .entry(field_of(&parameter.name))
            .or_default()
            .push(parameter.value);
    }
    groups.into_iter().all(|(field, values)| {
        row.get(&field)
            .is_some_and(|value| values.iter().any(|want| want == value))
    })
}

fn field_of(name: &str) -> String {
    match name.rsplit_once('_') {
        Some((head, tail)) if tail.chars().all(|c| c.is_ascii_digit()) => head.to_string(),
        _ => name.to_string(),
    }
}
