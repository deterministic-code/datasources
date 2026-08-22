use crate::parameter::Parameter;
use crate::value::Value;
use std::collections::HashMap;

pub fn parameters_to_map(parameters: Option<&[Parameter]>) -> HashMap<String, Value> {
    match parameters {
        None => HashMap::new(),
        Some(items) => items
            .iter()
            .map(|parameter| (parameter.name.clone(), parameter.value.clone()))
            .collect(),
    }
}
