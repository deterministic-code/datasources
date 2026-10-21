use crate::statement_renderer::quote_identifiers;
use datasources::Value;
use std::collections::HashMap;
use tokio_postgres::types::Type;

pub fn to_postgres(sql: &str, params: &HashMap<String, Value>) -> (String, Vec<Value>) {
    let sql = quote_identifiers(sql);
    let mut names = Vec::<String>::new();
    let mut out = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            i += 1;
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            let index = match names.iter().position(|n| n == &name) {
                Some(index) => index,
                None => {
                    names.push(name);
                    names.len() - 1
                }
            };
            out.push('$');
            out.push_str(&(index + 1).to_string());
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    let values = names
        .iter()
        .map(|name| params.get(name).cloned().unwrap_or(Value::Null))
        .collect();
    (out, values)
}

pub fn param_type(value: &Value) -> Type {
    match value {
        Value::Bool(_) => Type::BOOL,
        Value::Int(number) if *number >= i32::MIN as i64 && *number <= i32::MAX as i64 => Type::INT4,
        Value::Int(_) => Type::INT8,
        Value::Float(_) => Type::FLOAT8,
        Value::DateTime(_) => Type::TIMESTAMPTZ,
        Value::Uuid(_) => Type::UUID,
        Value::Decimal(_) => Type::NUMERIC,
        Value::Binary(_) => Type::BYTEA,
        _ => Type::UNKNOWN,
    }
}

pub fn param_types(values: &[Value]) -> Vec<Type> {
    values.iter().map(param_type).collect()
}
