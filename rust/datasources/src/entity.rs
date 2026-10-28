use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use chrono::{DateTime, Utc};

pub trait Entity: Clone + Send + Sync + 'static {
    fn to_row(&self) -> Row;
    fn from_row(row: Row) -> Result<Self>
    where
        Self: Sized;
    fn children(&self, property: &str) -> Vec<Row> {
        let _ = property;
        Vec::new()
    }
    fn set_children(&mut self, property: &str, rows: Vec<Row>) {
        let _ = (property, rows);
    }
    fn child_properties() -> &'static [&'static str] {
        &[]
    }
}

impl Entity for Row {
    fn to_row(&self) -> Row {
        self.clone()
    }

    fn from_row(row: Row) -> Result<Self> {
        Ok(row)
    }

    fn children(&self, property: &str) -> Vec<Row> {
        decode_rows(self.get(property))
    }

    fn set_children(&mut self, property: &str, rows: Vec<Row>) {
        self.insert(property.to_string(), encode_rows(&rows));
    }
}

pub fn stored_row<T: Entity>(item: &T) -> Row {
    let mut row = item.to_row();
    for name in T::child_properties() {
        row.insert((*name).to_string(), encode_rows(&item.children(name)));
    }
    row
}

pub trait ToCell {
    fn to_cell(&self) -> Value;
}

pub trait FromCell: Sized {
    fn from_cell(value: Option<&Value>) -> Self;
}

impl ToCell for bool {
    fn to_cell(&self) -> Value {
        Value::from(*self)
    }
}

impl ToCell for i16 {
    fn to_cell(&self) -> Value {
        Value::from(i64::from(*self))
    }
}

impl ToCell for i32 {
    fn to_cell(&self) -> Value {
        Value::from(*self)
    }
}

impl ToCell for i64 {
    fn to_cell(&self) -> Value {
        Value::from(*self)
    }
}

impl ToCell for String {
    fn to_cell(&self) -> Value {
        Value::from(self.clone())
    }
}

impl ToCell for DateTime<Utc> {
    fn to_cell(&self) -> Value {
        Value::from(*self)
    }
}

impl ToCell for Vec<u8> {
    fn to_cell(&self) -> Value {
        Value::from(self.clone())
    }
}

impl ToCell for uuid::Uuid {
    fn to_cell(&self) -> Value {
        Value::from(self.to_string())
    }
}

impl ToCell for rust_decimal::Decimal {
    fn to_cell(&self) -> Value {
        Value::from(*self)
    }
}

impl<T: ToCell> ToCell for Option<T> {
    fn to_cell(&self) -> Value {
        self.as_ref().map(ToCell::to_cell).unwrap_or(Value::Null)
    }
}

impl FromCell for bool {
    fn from_cell(value: Option<&Value>) -> Self {
        matches!(value, Some(Value::Bool(true)))
    }
}

impl FromCell for i16 {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Int(n)) => *n as i16,
            Some(other) => other.wire_text().parse().unwrap_or(0),
            None => 0,
        }
    }
}

impl FromCell for i32 {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Int(n)) => *n as i32,
            Some(other) => other.wire_text().parse().unwrap_or(0),
            None => 0,
        }
    }
}

impl FromCell for i64 {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Int(n)) => *n,
            Some(other) => other.wire_text().parse().unwrap_or(0),
            None => 0,
        }
    }
}

impl FromCell for String {
    fn from_cell(value: Option<&Value>) -> Self {
        value.map(Value::wire_text).unwrap_or_default()
    }
}

impl FromCell for DateTime<Utc> {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::DateTime(date)) => *date,
            _ => Utc::now(),
        }
    }
}

impl FromCell for Vec<u8> {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Binary(bytes)) => bytes.clone(),
            _ => Vec::new(),
        }
    }
}

impl FromCell for uuid::Uuid {
    fn from_cell(value: Option<&Value>) -> Self {
        value
            .map(Value::wire_text)
            .and_then(|text| text.parse().ok())
            .unwrap_or_else(uuid::Uuid::nil)
    }
}

impl FromCell for rust_decimal::Decimal {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            Some(Value::Decimal(n)) => *n,
            Some(other) => other.wire_text().parse().unwrap_or_default(),
            None => rust_decimal::Decimal::ZERO,
        }
    }
}

impl<T: FromCell> FromCell for Option<T> {
    fn from_cell(value: Option<&Value>) -> Self {
        match value {
            None | Some(Value::Null) => None,
            other => Some(T::from_cell(other)),
        }
    }
}

pub fn entity_from_row<T: Entity>(row: Row) -> Result<T> {
    let mut item = T::from_row(row.clone())?;
    for name in T::child_properties() {
        item.set_children(name, decode_rows(row.get(*name)));
    }
    Ok(item)
}
