use std::collections::HashMap;

pub fn load(path: &str) -> HashMap<String, String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|text| parse(&text))
        .unwrap_or_default()
}

pub fn parse(text: &str) -> HashMap<String, String> {
    let mut options = HashMap::new();
    for raw in text.lines() {
        let line = raw.trim_start();
        if line.is_empty() || line.starts_with('#') || !line.contains('=') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if !key.is_empty() {
            options.insert(key.to_string(), value.to_string());
        }
    }
    options
}
