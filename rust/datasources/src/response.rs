use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub result: Value,
    pub rows_affected: i64,
    pub parameters: Option<Vec<Parameter>>,
}

impl Response {
    pub fn new(
        result: impl Into<Value>,
        rows_affected: i64,
        parameters: Option<Vec<Parameter>>,
    ) -> Self {
        let result = result.into();
        let result = match (&result, &parameters) {
            (Value::Null, Some(list)) => list
                .iter()
                .find(|parameter| parameter.direction == ParameterDirection::ReturnValue)
                .map(|parameter| parameter.value.clone())
                .unwrap_or(result),
            _ => result,
        };
        Self {
            result,
            rows_affected,
            parameters,
        }
    }
}
