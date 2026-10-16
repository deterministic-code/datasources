use serde_yaml::{Mapping, Sequence, Value};
use std::collections::{HashMap, HashSet};

pub fn parse(yaml: &str) -> Result<Value, serde_yaml::Error> {
    if yaml.trim().is_empty() {
        return Ok(Value::Null);
    }
    serde_yaml::from_str(yaml)
}

pub fn yaml(value: &Value) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(value)
}

pub fn apply_document(
    before: &HashMap<String, Value>,
    doc: &Mapping,
) -> HashMap<String, Value> {
    let mut output = before.clone();
    for (key, envelope) in doc {
        let path = match key.as_str() {
            Some(text) => text.to_string(),
            None => continue,
        };
        match apply_file(output.get(&path), envelope) {
            None => {
                output.remove(&path);
            }
            Some(next) => {
                output.insert(path, next);
            }
        }
    }
    output
}

pub fn as_document(value: &Value, source: &str) -> Result<Mapping, String> {
    match value.as_mapping() {
        Some(map) => Ok(map.clone()),
        None => Err(format!("{source}: expected a mapping")),
    }
}

pub fn equal(left: &Value, right: &Value) -> bool {
    left == right
}

fn apply_file(before: Option<&Value>, envelope: &Value) -> Option<Value> {
    if envelope.is_null() {
        return before.cloned();
    }
    if let Some(map) = envelope.as_mapping() {
        if is_true(map, "removed") && map.len() == 1 {
            return None;
        }
        if is_true(map, "added") {
            return map.get(Value::String("content".into())).cloned();
        }
    }
    Some(apply(before.unwrap_or(&Value::Null), envelope))
}

fn apply(before: &Value, next: &Value) -> Value {
    if next.is_null() {
        return before.clone();
    }
    if is_keyed_collection(before) && is_keyed_split(next) {
        return apply_keyed(before.as_sequence().unwrap(), next.as_mapping().unwrap());
    }
    if before.as_mapping().is_some() && next.as_mapping().is_some() {
        return apply_object(before.as_mapping().unwrap(), next.as_mapping().unwrap());
    }
    next.clone()
}

fn apply_keyed(before_val: &Sequence, split: &Mapping) -> Value {
    let removed: HashSet<String> = strings(split, "removed").into_iter().collect();
    let mut changed = HashMap::new();
    if let Some(items) = split.get(Value::String("changed".into())).and_then(Value::as_sequence) {
        for entry in items {
            if let Some((key, value)) = first_pair(entry) {
                changed.insert(key, value.clone());
            }
        }
    }
    let mut output = Sequence::new();
    for entry in before_val {
        let Some((key, value)) = first_pair(entry) else {
            continue;
        };
        if removed.contains(&key) {
            continue;
        }
        output.push(match changed.get(&key) {
            Some(nested) => mapping_of(&key, apply(value, nested)),
            None => entry.clone(),
        });
    }
    if let Some(items) = split.get(Value::String("added".into())).and_then(Value::as_sequence) {
        output.extend(items.iter().cloned());
    }
    Value::Sequence(output)
}

fn apply_object(before_val: &Mapping, next: &Mapping) -> Value {
    let removed: HashSet<String> = strings(next, "removed").into_iter().collect();
    let mut output = Mapping::new();
    for (key, value) in before_val {
        let Some(name) = key.as_str() else { continue };
        if removed.contains(name) {
            continue;
        }
        output.insert(
            key.clone(),
            match next.get(key) {
                Some(nested) => apply(value, nested),
                None => value.clone(),
            },
        );
    }
    for (key, value) in next {
        let Some(name) = key.as_str() else { continue };
        if name == "removed" && next.get(key).and_then(Value::as_sequence).is_some() {
            continue;
        }
        if !before_val.contains_key(key) {
            output.insert(key.clone(), value.clone());
        }
    }
    Value::Mapping(output)
}

fn is_keyed_split(value: &Value) -> bool {
    let Some(map) = value.as_mapping() else {
        return false;
    };
    let allowed = ["added", "changed", "removed"];
    map.keys().all(|key| key.as_str().map(|name| allowed.contains(&name)).unwrap_or(false))
}

fn is_keyed_collection(value: &Value) -> bool {
    match value.as_sequence() {
        Some(items) if !items.is_empty() => items.iter().all(|item| {
            item.as_mapping().is_some_and(|map| map.len() == 1)
        }),
        _ => false,
    }
}

fn strings(map: &Mapping, key: &str) -> Vec<String> {
    map.get(Value::String(key.into()))
        .and_then(Value::as_sequence)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn first_pair(entry: &Value) -> Option<(String, &Value)> {
    let map = entry.as_mapping()?;
    let (key, value) = map.iter().next()?;
    Some((key.as_str()?.to_string(), value))
}

fn mapping_of(key: &str, value: Value) -> Value {
    let mut map = Mapping::new();
    map.insert(Value::String(key.into()), value);
    Value::Mapping(map)
}

fn is_true(map: &Mapping, key: &str) -> bool {
    match map.get(Value::String(key.into())) {
        Some(Value::Bool(true)) => true,
        Some(Value::String(text)) if text == "true" => true,
        _ => false,
    }
}
