use std::collections::HashMap;

pub fn identity_where_text<T>(obj: &HashMap<String, T>) -> String {
    if obj.contains_key("id") {
        return "id = $id".to_string();
    }
    if obj.is_empty() {
        return "1 = 1".to_string();
    }
    let mut keys: Vec<&String> = obj.keys().collect();
    keys.sort();
    keys.into_iter()
        .map(|key| format!("{key} = ${key}"))
        .collect::<Vec<_>>()
        .join(" AND ")
}
