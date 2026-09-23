use std::collections::HashMap;

pub fn load(path: &str) -> HashMap<String, String> {
    std::fs::read_to_string(path)
        .map(|text| parse(&text))
        .unwrap_or_default()
}

pub fn parse(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|raw| match raw.trim_start().split_once('=') {
            Some((key, value)) if !key.starts_with('#') && !key.trim().is_empty() => {
                Some((key.trim().to_string(), value.to_string()))
            }
            _ => None,
        })
        .collect()
}
