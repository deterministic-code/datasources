use chrono::DateTime;
use datasources::{IsolationLevel, Value};
use std::collections::HashMap;
use tokio_postgres::types::Type;

fn quote_mixed_case(sql: &str) -> String {
    let chars: Vec<char> = sql.chars().collect();
    let mut out = String::with_capacity(sql.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$'
            && i + 1 < chars.len()
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            out.push('$');
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                out.push(chars[i]);
                i += 1;
            }
            continue;
        }
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            out.push(quote);
            i += 1;
            while i < chars.len() {
                out.push(chars[i]);
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if chars[i].is_ascii_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let token: String = chars[start..i].iter().collect();
            if token.chars().any(|ch| ch.is_ascii_uppercase()) && token.chars().any(|ch| ch.is_ascii_lowercase()) {
                out.push('"');
                out.push_str(&token);
                out.push('"');
            } else {
                out.push_str(&token);
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

pub fn to_postgres(sql: &str, params: &HashMap<String, Value>) -> (String, Vec<Value>) {
    let sql = quote_mixed_case(sql);
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

pub fn is_rfc3339(text: &str) -> bool {
    DateTime::parse_from_rfc3339(text).is_ok()
}

pub fn is_base64(text: &str) -> bool {
    let text = text.trim();
    text.len() >= 4
        && text.len() % 4 == 0
        && text.contains('=')
        && text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
        && base64::Engine::decode(&base64::engine::general_purpose::STANDARD, text).is_ok()
}

pub fn param_type(value: &Value) -> Type {
    match value {
        Value::Bool(_) => Type::BOOL,
        Value::Int(_) => Type::INT8,
        Value::Float(_) => Type::FLOAT8,
        Value::DateTime(_) => Type::TIMESTAMPTZ,
        Value::Uuid(_) => Type::UUID,
        Value::Decimal(_) => Type::NUMERIC,
        Value::Binary(_) => Type::BYTEA,
        Value::Text(text) if is_rfc3339(text) => Type::TIMESTAMPTZ,
        Value::Text(text) if is_base64(text) => Type::BYTEA,
        _ => Type::UNKNOWN,
    }
}

pub fn param_types(values: &[Value]) -> Vec<Type> {
    values.iter().map(param_type).collect()
}

pub fn isolation_sql(isolation: IsolationLevel) -> &'static str {
    match isolation {
        IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel::ReadCommitted => "READ COMMITTED",
        IsolationLevel::RepeatableRead => "REPEATABLE READ",
        IsolationLevel::Serializable => "SERIALIZABLE",
    }
}
