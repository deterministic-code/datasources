use crate::parameter_direction::ParameterDirection;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<T = Value> {
    pub name: String,
    pub value: T,
    pub direction: ParameterDirection,
}

impl Parameter<Value> {
    pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            direction: ParameterDirection::Input,
        }
    }

    pub fn output(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Value::Null,
            direction: ParameterDirection::Output,
        }
    }

    pub fn return_value(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Value::Null,
            direction: ParameterDirection::ReturnValue,
        }
    }

    pub fn with_direction(
        name: impl Into<String>,
        value: impl Into<Value>,
        direction: ParameterDirection,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            direction,
        }
    }
}

impl<T> Parameter<T> {
    pub fn with(name: impl Into<String>, value: T) -> Self {
        Self {
            name: name.into(),
            value,
            direction: ParameterDirection::Input,
        }
    }
}
