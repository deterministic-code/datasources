use crate::converters::value_converter::ValueConverter;
use crate::result::Result;
use crate::value::Value;
use std::collections::HashMap;

fn identity(value: &Value) -> Result<Value> {
    Ok(value.clone())
}

fn bool_convert(value: &Value) -> Result<Value> {
    Ok(match value {
        Value::Bool(flag) => Value::Bool(*flag),
        Value::Int(number) => Value::Bool(*number != 0),
        Value::Text(text) => Value::Bool(text == "1" || text.eq_ignore_ascii_case("true")),
        Value::Null => Value::Null,
        other => other.clone(),
    })
}

pub fn core_types() -> HashMap<String, ValueConverter> {
    HashMap::from([
        (
            "bool".into(),
            ValueConverter::new(bool_convert, identity),
        ),
        ("datetime".into(), ValueConverter::new(identity, identity)),
        ("guid".into(), ValueConverter::new(identity, identity)),
        ("decimal".into(), ValueConverter::new(identity, identity)),
        ("binary".into(), ValueConverter::new(identity, identity)),
    ])
}
