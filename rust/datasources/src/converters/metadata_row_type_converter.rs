use crate::converters::type_converter::TypeConverter;
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::from_schema::{FromSchema, SchemaObject};
use crate::mappers::{IdentityMapper, Mapper};
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use base64::Engine;
use chrono::{DateTime, Utc};
use facet::{Def, Field, ScalarType, Shape, Type, UserType};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;

pub struct MetaDataRowTypeConverter<T: FromSchema = Row> {
    columns: Vec<String>,
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    references: Vec<String>,
    reference_columns: Box<dyn Mapper<String, String> + Send + Sync>,
    pub should_coerce_id: bool,
    entity: PhantomData<T>,
}

impl<T: FromSchema> MetaDataRowTypeConverter<T> {
    pub fn new(columns: Vec<String>) -> Self {
        Self {
            columns,
            fields: HashMap::new(),
            references: Vec::new(),
            reference_columns: Box::new(IdentityMapper::new()),
            should_coerce_id: true,
            entity: PhantomData,
        }
    }

    pub fn with_references(
        mut self,
        references: Vec<String>,
        columns: Box<dyn Mapper<String, String> + Send + Sync>,
    ) -> Self {
        self.references = references;
        self.reference_columns = columns;
        self
    }

    pub fn with_fields(
        mut self,
        fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    ) -> Self {
        self.fields.extend(fields);
        self
    }

    fn apply(&self, row: &Row, reverse: bool) -> Result<Row> {
        let names = match self.columns.is_empty() {
            true => row.keys().cloned().collect(),
            false => self.columns.clone(),
        };
        names
            .into_iter()
            .filter_map(|key| row.get(&key).cloned().map(|value| (key, value)))
            .map(|(key, value)| {
                let cell = match self.fields.get(&key) {
                    Some(field) if reverse => field.reverse_convert(&value)?,
                    Some(field) => field.convert(&value)?,
                    None => value,
                };
                let key = if reverse && self.references.iter().any(|name| name == &key) {
                    Mapper::map(self.reference_columns.as_ref(), key)
                } else {
                    key
                };
                Ok((key, cell))
            })
            .collect()
    }
}

impl MetaDataRowTypeConverter<Row> {
    pub fn convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::convert(self, row)
    }

    pub fn reverse_convert(&self, row: &Row) -> Result<Row> {
        TypeConverter::reverse_convert(self, row)
    }
}

impl TypeConverter<Row, Row> for MetaDataRowTypeConverter<Row> {
    fn convert(&self, from: &Row) -> Result<Row> {
        self.apply(from, false)
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        self.apply(to, true)
    }
}

impl<T: SchemaObject> TypeConverter<T, Row> for MetaDataRowTypeConverter<T> {
    fn convert(&self, from: &Row) -> Result<T> {
        let cells = self.apply(from, false)?;
        let coerced = coerce_struct(T::SHAPE, &cells, self.should_coerce_id)?;
        T::from_schema(coerced, &Schema::default())
    }

    fn reverse_convert(&self, to: &T) -> Result<Row> {
        self.apply(&to.to_schema(&Schema::default())?, true)
    }
}

pub(crate) fn coerce_entity(shape: &'static Shape, row: &Row, coerce_id: bool) -> Result<Row> {
    coerce_struct(shape, row, coerce_id)
}

fn coerce_struct(shape: &'static Shape, row: &Row, coerce_id: bool) -> Result<Row> {
    let Ok(fields) = struct_fields(shape) else {
        return Ok(row.clone());
    };
    let mut out = row.clone();
    for field in fields {
        let name = field.name;
        let field_shape = field.shape();
        if is_map(field_shape) {
            out.insert(name.to_string(), coerce_encoded(field_shape, row.get(name), coerce_id, true)?);
            continue;
        }
        if is_list(field_shape) && !is_bytes(field_shape) {
            out.insert(name.to_string(), coerce_encoded(field_shape, row.get(name), coerce_id, false)?);
            continue;
        }
        let cell = row.get(name).cloned().unwrap_or(Value::Null);
        out.insert(name.to_string(), coerce_cell(&cell, field_shape, coerce_id)?);
    }
    Ok(out)
}

fn coerce_encoded(shape: &'static Shape, cell: Option<&Value>, coerce_id: bool, map: bool) -> Result<Value> {
    let rows = decode_rows(cell);
    if rows.is_empty() {
        return Ok(cell.cloned().unwrap_or(Value::Null));
    }
    let mut next = Vec::new();
    for row in rows {
        if map {
            let (key_shape, value_shape) = map_kv(shape)?;
            let mut stored = Row::new();
            let key = row.get("key").cloned().unwrap_or(Value::Null);
            stored.insert("key".into(), coerce_cell(&key, key_shape, coerce_id)?);
            if struct_fields(value_shape).is_ok() {
                let coerced = coerce_struct(value_shape, &row, coerce_id)?;
                for (name, value) in coerced {
                    if name != "key" {
                        stored.insert(name, value);
                    }
                }
            } else {
                let value = row.get("value").cloned().unwrap_or(Value::Null);
                stored.insert("value".into(), coerce_cell(&value, value_shape, coerce_id)?);
            }
            next.push(stored);
        } else if struct_fields(list_element(shape)?).is_ok() {
            next.push(coerce_struct(list_element(shape)?, &row, coerce_id)?);
        } else {
            next.push(row);
        }
    }
    Ok(encode_rows(&next))
}

fn coerce_cell(cell: &Value, shape: &'static Shape, coerce_id: bool) -> Result<Value> {
    if is_option(shape) {
        let inner = option_inner(shape)?;
        if *cell == Value::Null || empty_text_non_string(cell, inner) {
            return Ok(Value::Null);
        }
        return coerce_cell(cell, inner, coerce_id);
    }
    if is_bytes(shape) {
        return match cell {
            Value::Binary(bytes) => Ok(Value::Binary(bytes.clone())),
            Value::Text(text) if !text.is_empty() => match base64::engine::general_purpose::STANDARD.decode(text) {
                Ok(bytes) => Ok(Value::Binary(bytes)),
                Err(error) => Err(DatasourceError::message(error.to_string())),
            },
            other => Ok(other.clone()),
        };
    }
    let ident = shape.type_identifier;
    match (cell, ScalarType::try_from_shape(shape), ident) {
        (Value::Bool(flag), Some(ScalarType::Bool), _) => Ok(Value::Bool(*flag)),
        (Value::Int(n), Some(ScalarType::Bool), _) => Ok(Value::Bool(*n != 0)),
        (Value::Text(text), Some(ScalarType::Bool), _) if text.is_empty() => Ok(Value::Bool(false)),
        (Value::Int(n), Some(ScalarType::I64), _) => Ok(Value::Int(*n)),
        (Value::Int(n), Some(ScalarType::I8), _) => narrow(*n, i8::MIN as i64, i8::MAX as i64, coerce_id),
        (Value::Int(n), Some(ScalarType::I16), _) => narrow(*n, i16::MIN as i64, i16::MAX as i64, coerce_id),
        (Value::Int(n), Some(ScalarType::I32), _) => narrow(*n, i32::MIN as i64, i32::MAX as i64, coerce_id),
        (Value::Int(n), Some(ScalarType::U32), _) => narrow(*n, 0, u32::MAX as i64, coerce_id),
        (Value::Text(text), Some(ScalarType::I8 | ScalarType::I16 | ScalarType::I32 | ScalarType::I64 | ScalarType::U32), _)
            if text.is_empty() =>
        {
            Ok(Value::Int(0))
        }
        (Value::Text(text), Some(ScalarType::String), _) => Ok(Value::Text(text.clone())),
        (Value::Text(text), _, "String") => Ok(Value::Text(text.clone())),
        (Value::Int(n), _, "String") => Ok(Value::Text(n.to_string())),
        (Value::Uuid(id), _, "String") => Ok(Value::Text(id.to_string())),
        (Value::DateTime(date), _, "String") => Ok(Value::Text(Value::iso_datetime(*date))),
        (Value::Decimal(number), _, "String") => Ok(Value::Text(number.to_string())),
        (Value::Text(text), _, ident) if ident.contains("DateTime") => match DateTime::parse_from_rfc3339(text) {
            Ok(parsed) => Ok(Value::DateTime(parsed.with_timezone(&Utc))),
            Err(error) => Err(DatasourceError::message(error.to_string())),
        },
        (Value::DateTime(date), _, ident) if ident.contains("DateTime") => Ok(Value::DateTime(*date)),
        (Value::Text(text), _, ident) if ident.contains("Uuid") => match Uuid::parse_str(text) {
            Ok(id) => Ok(Value::Uuid(id)),
            Err(error) => Err(DatasourceError::message(error.to_string())),
        },
        (Value::Uuid(id), _, ident) if ident.contains("Uuid") => Ok(Value::Uuid(*id)),
        (Value::Decimal(number), _, ident) if ident.contains("Decimal") => Ok(Value::Decimal(*number)),
        (Value::Text(text), _, ident) if ident.contains("Decimal") => match Decimal::from_str(text) {
            Ok(number) => Ok(Value::Decimal(number)),
            Err(error) => Err(DatasourceError::message(error.to_string())),
        },
        (Value::Int(n), _, ident) if ident.contains("Decimal") => Ok(Value::Decimal(Decimal::from(*n))),
        (Value::Float(n), _, ident) if ident.contains("Decimal") => match Decimal::from_str(&n.to_string()) {
            Ok(number) => Ok(Value::Decimal(number)),
            Err(error) => Err(DatasourceError::message(error.to_string())),
        },
        (other, _, _) => Ok(other.clone()),
    }
}

fn narrow(n: i64, min: i64, max: i64, coerce_id: bool) -> Result<Value> {
    if !coerce_id {
        return Ok(Value::Null);
    }
    if n < min || n > max {
        return Err(DatasourceError::message(format!("{n} does not fit")));
    }
    Ok(Value::Int(n))
}

fn struct_fields(shape: &'static Shape) -> Result<&'static [Field]> {
    match shape.ty {
        Type::User(UserType::Struct(st)) => Ok(st.fields),
        _ => Err(DatasourceError::message(format!("{} is not a struct", shape.type_identifier))),
    }
}

fn is_option(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::Option(_)) || shape.type_identifier == "Option"
}

fn is_bytes(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::List(_)) && list_element(shape).is_ok_and(|inner| inner.type_identifier == "u8")
}

fn is_map(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::Map(_)) || shape.type_identifier.contains("HashMap")
}

fn is_list(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::List(_)) || shape.type_identifier.starts_with("Vec")
}

fn map_kv(shape: &'static Shape) -> Result<(&'static Shape, &'static Shape)> {
    match shape.def {
        Def::Map(map) => Ok((map.k, map.v)),
        _ => Err(DatasourceError::message(format!("{} is not a map", shape.type_identifier))),
    }
}

fn list_element(shape: &'static Shape) -> Result<&'static Shape> {
    match shape.def {
        Def::List(list) => Ok(list.t),
        _ => Err(DatasourceError::message(format!("{} is not a list", shape.type_identifier))),
    }
}

fn option_inner(shape: &'static Shape) -> Result<&'static Shape> {
    match shape.def {
        Def::Option(option) => Ok(option.t()),
        _ => Err(DatasourceError::message(format!("{} is not an option", shape.type_identifier))),
    }
}

fn empty_text_non_string(cell: &Value, inner: &'static Shape) -> bool {
    matches!(cell, Value::Text(text) if text.is_empty())
        && ScalarType::try_from_shape(inner) != Some(ScalarType::String)
        && inner.type_identifier != "String"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pairs: &[(&str, Value)]) -> Row {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), value.clone()))
            .collect()
    }

    #[test]
    fn copies_schema_columns_and_drops_the_rest() {
        let converter = MetaDataRowTypeConverter::new(vec!["id".into(), "name".into()]);
        let row = record(&[
            ("id", Value::Int(1)),
            ("name", Value::from("ada")),
            ("extra", Value::from("no")),
        ]);
        let converted = converter.convert(&row).unwrap();
        assert_eq!(converted.get("id"), Some(&Value::Int(1)));
        assert_eq!(converted.get("name"), Some(&Value::from("ada")));
        assert_eq!(converted.get("extra"), None);
        assert_eq!(converter.reverse_convert(&converted).unwrap(), converted);
    }

    #[test]
    fn empty_schema_copies_every_cell() {
        let converter = MetaDataRowTypeConverter::new(Vec::new());
        let row = record(&[("id", Value::Int(1)), ("label", Value::from("a"))]);
        assert_eq!(converter.convert(&row).unwrap(), row);
    }

    #[derive(Clone, Debug, PartialEq, Default, facet::Facet)]
    #[facet(crate = facet)]
    struct Priced {
        score: String,
    }

    impl SchemaObject for Priced {}

    #[derive(Clone, Debug, PartialEq, Default, facet::Facet)]
    #[facet(crate = facet)]
    struct Keyed {
        uuid: String,
    }

    impl SchemaObject for Keyed {}

    #[derive(Clone, Debug, PartialEq, Default, facet::Facet)]
    #[facet(crate = facet)]
    struct Counted {
        id: i32,
    }

    impl SchemaObject for Counted {}

    #[derive(Clone, Debug, PartialEq, Default, facet::Facet)]
    #[facet(crate = facet)]
    struct Dated {
        created: chrono::DateTime<chrono::Utc>,
    }

    impl SchemaObject for Dated {}

    #[test]
    fn coerces_decimal_onto_string() {
        let converter = MetaDataRowTypeConverter::<Priced>::new(Vec::new());
        let row = record(&[("score", Value::Decimal("10.50".parse().unwrap()))]);
        let priced = TypeConverter::convert(&converter, &row).unwrap();
        assert_eq!(priced.score, "10.50");
    }

    #[test]
    fn coerces_guid_onto_string() {
        let id = uuid::Uuid::parse_str("54ef41f2-0cb4-4c72-97a7-776fd183f5ed").unwrap();
        let converter = MetaDataRowTypeConverter::<Keyed>::new(Vec::new());
        let row = record(&[("uuid", Value::Uuid(id))]);
        let keyed = TypeConverter::convert(&converter, &row).unwrap();
        assert_eq!(keyed.uuid, id.to_string());
    }

    #[test]
    fn coerces_int_property_when_should_coerce_id() {
        let converter = MetaDataRowTypeConverter::<Counted>::new(Vec::new());
        let row = record(&[("id", Value::Int(1))]);
        assert_eq!(TypeConverter::convert(&converter, &row).unwrap().id, 1);
        let overflow = record(&[("id", Value::Int(i32::MAX as i64 + 1))]);
        assert!(TypeConverter::convert(&converter, &overflow).is_err());
    }

    #[test]
    fn leaves_int_property_when_cell_is_long() {
        let mut converter = MetaDataRowTypeConverter::<Counted>::new(Vec::new());
        converter.should_coerce_id = false;
        let row = record(&[("id", Value::Int(1))]);
        assert_eq!(TypeConverter::convert(&converter, &row).unwrap().id, 0);
    }

    #[test]
    fn coerces_iso_text_onto_datetime() {
        let converter = MetaDataRowTypeConverter::<Dated>::new(Vec::new());
        let row = record(&[("created", Value::from("2020-01-02T03:04:05.000Z"))]);
        let dated = TypeConverter::convert(&converter, &row).unwrap();
        assert_eq!(
            dated.created.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "2020-01-02T03:04:05.000Z"
        );
    }
}
