use crate::converters::fail::fail;
use crate::converters::field_converter::FieldConverter;
use crate::value::Value;

pub fn boolean_field_converter(storage: &str) -> FieldConverter {
    let native = storage == "native";
    FieldConverter::new(
        |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Bool(flag) => Ok(Value::Bool(*flag)),
            Value::Int(0) => Ok(Value::Bool(false)),
            Value::Int(1) => Ok(Value::Bool(true)),
            other => Err(fail(other, "boolean")),
        },
        move |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Bool(flag) if native => Ok(Value::Bool(*flag)),
            Value::Bool(flag) => Ok(Value::Int(if *flag { 1 } else { 0 })),
            Value::Int(0) => Ok(if native {
                Value::Bool(false)
            } else {
                Value::Int(0)
            }),
            Value::Int(1) => Ok(if native {
                Value::Bool(true)
            } else {
                Value::Int(1)
            }),
            other => Err(fail(other, "boolean")),
        },
    )
}
