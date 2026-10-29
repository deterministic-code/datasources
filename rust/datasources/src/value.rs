use chrono::{DateTime, SecondsFormat, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    DateTime(DateTime<Utc>),
    Uuid(Uuid),
    Decimal(Decimal),
    Binary(Vec<u8>),
}

impl Value {
    pub fn iso_datetime(value: DateTime<Utc>) -> String {
        value.to_rfc3339_opts(SecondsFormat::Millis, true)
    }

    pub fn wire_text(&self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(flag) => flag.to_string(),
            Value::Int(number) => number.to_string(),
            Value::Float(number) => number.to_string(),
            Value::Text(text) => text.clone(),
            Value::DateTime(date) => Self::iso_datetime(*date),
            Value::Uuid(id) => id.to_string(),
            Value::Decimal(number) => number.to_string(),
            Value::Binary(bytes) => {
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
            }
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Int(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value.map(Into::into).unwrap_or(Self::Null)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(value: DateTime<Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl From<Uuid> for Value {
    fn from(value: Uuid) -> Self {
        Self::Uuid(value)
    }
}

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Self::Decimal(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Self::Binary(value)
    }
}

impl TryFrom<Value> for bool {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Bool(flag) => Ok(flag),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for i16 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Int(n) => Ok(n as i16),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Int(n) => Ok(n as i32),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Int(n) => Ok(n),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Text(text) => Ok(text),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for DateTime<Utc> {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::DateTime(date) => Ok(date),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for Uuid {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Uuid(id) => Ok(id),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for Decimal {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Decimal(n) => Ok(n),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Binary(bytes) => Ok(bytes),
            _ => Err(()),
        }
    }
}

impl<T: TryFrom<Value, Error = ()>> TryFrom<Value> for Option<T> {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, ()> {
        match value {
            Value::Null => Ok(None),
            other => T::try_from(other).map(Some),
        }
    }
}
