use crate::value::Value;
use std::collections::HashMap;

pub fn identity_filter(obj: &HashMap<String, Value>) -> HashMap<String, Value> {
    if let Some(id) = obj.get("id") {
        let mut filter = HashMap::new();
        filter.insert("id".to_string(), id.clone());
        return filter;
    }
    obj.clone()
}
