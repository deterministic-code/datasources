use serde_yaml::Value;

#[derive(Clone)]
pub struct YamlCursor {
    value: Value,
}

impl YamlCursor {
    pub fn from_yaml(text: &str) -> Self {
        Self {
            value: serde_yaml::from_str(text).unwrap_or(Value::Null),
        }
    }

    pub fn child(&self, key: &str) -> Self {
        Self {
            value: self.at(key).cloned().unwrap_or(Value::Null),
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.at(key).is_some()
    }

    pub fn str(&self, key: &str) -> Option<String> {
        match self.at(key)? {
            Value::String(text) => Some(text.clone()),
            Value::Bool(flag) => Some(if *flag { "true".into() } else { "false".into() }),
            other => Some(yaml_to_string(other)),
        }
    }

    pub fn bool(&self, key: &str) -> bool {
        match self.at(key) {
            Some(Value::Bool(true)) => true,
            Some(Value::String(text)) => text == "true" || text == "True",
            _ => false,
        }
    }

    pub fn literal(&self, key: &str) -> Option<Value> {
        let value = self.at(key)?;
        is_literal(value).then(|| value.clone())
    }

    pub fn strings(&self, key: &str) -> Vec<String> {
        match self.at(key) {
            Some(Value::Sequence(items)) => items.iter().map(yaml_to_string).collect(),
            Some(Value::String(text)) => vec![text.clone()],
            _ => vec![],
        }
    }

    pub fn items(&self) -> Vec<YamlCursor> {
        match &self.value {
            Value::Sequence(items) => items.iter().cloned().map(|value| Self { value }).collect(),
            _ => vec![],
        }
    }

    pub fn named_items(&self) -> Vec<(String, YamlCursor)> {
        self.items()
            .into_iter()
            .filter_map(|item| {
                let map = item.value.as_mapping()?;
                let (k, v) = map.iter().next()?;
                Some((yaml_to_string(k), Self { value: v.clone() }))
            })
            .collect()
    }

    pub fn named_list(&self, key: &str) -> Vec<(String, YamlCursor)> {
        self.child(key).named_items()
    }

    pub fn map_record(&self) -> bool {
        self.value.as_mapping().is_some()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    fn at(&self, key: &str) -> Option<&Value> {
        let map = self.value.as_mapping()?;
        map.iter().find_map(|(k, v)| (yaml_to_string(k) == key).then_some(v))
    }
}

fn is_literal(value: &Value) -> bool {
    matches!(
        value,
        Value::Null | Value::String(_) | Value::Bool(_) | Value::Number(_)
    ) || matches!(value, Value::Sequence(items) if items.is_empty())
        || matches!(value, Value::Mapping(map) if map.is_empty())
}

fn yaml_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => if *flag { "true" } else { "false" }.into(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => format!("{other:?}"),
    }
}
