use crate::datasource_error::DatasourceError;
use crate::datasource_url_parser::DatasourceUrlParser;
use crate::result::Result;
use std::collections::HashMap;

pub fn defaults(options: HashMap<String, String>) -> Result<()> {
    register_listed(&options)
}

pub fn register_listed(options: &HashMap<String, String>) -> Result<()> {
    for name in installed(options)? {
        if crate::registered().iter().any(|existing| existing == &name) {
            continue;
        }
        return Err(DatasourceError::not_installed(name));
    }
    Ok(())
}

fn installed(options: &HashMap<String, String>) -> Result<Vec<String>> {
    let mut names = Vec::new();
    if let Some(value) = get(options, "INSTALLED_PROVIDERS") {
        for name in value.split(',') {
            let name = name.trim();
            if !name.is_empty() && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        }
    }
    let Some(url) = get(options, "DATASOURCE_URL") else {
        return Ok(names);
    };
    let provider = DatasourceUrlParser::parse(url)
        .map(|parsed| parsed.provider)
        .ok_or_else(|| DatasourceError::message("DATASOURCE_URL has no provider"))?;
    if !names.iter().any(|existing| existing == &provider) {
        names.push(provider);
    }
    Ok(names)
}

fn get<'a>(options: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    options
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}
