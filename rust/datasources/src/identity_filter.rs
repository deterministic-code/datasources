use std::collections::HashMap;

pub fn identity_filter<T: Clone>(obj: &HashMap<String, T>) -> HashMap<String, T> {
    if let Some(id) = obj.get("id") {
        let mut filter = HashMap::new();
        filter.insert("id".to_string(), id.clone());
        return filter;
    }
    obj.clone()
}
