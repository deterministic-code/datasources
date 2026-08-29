use crate::parameter::Parameter;
use std::collections::HashMap;

pub fn parameters_to_map<T: Clone>(parameters: Option<&[Parameter<T>]>) -> HashMap<String, T> {
    match parameters {
        None => HashMap::new(),
        Some(items) => items
            .iter()
            .map(|parameter| (parameter.name.clone(), parameter.value.clone()))
            .collect(),
    }
}
