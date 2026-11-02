use crate::converters::type_converter::TypeConverter;
use crate::datasource_error::DatasourceError;
use crate::mappers::{IdentityMapper, Mapper};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use facet::{Def, Facet, Field, HasFields, Partial, Peek, ScalarType, Shape, Type, UserType};
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait SchemaObject: Facet<'static> + Clone + Default + Send + Sync + 'static {}

pub struct MetaDataRowTypeConverter<T = Row> {
    pub fields: HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
    references: Vec<String>,
    reference_columns: Box<dyn Mapper<String, String> + Send + Sync>,
    pub should_coerce_id: bool,
    entity: PhantomData<T>,
}

impl<T> MetaDataRowTypeConverter<T> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            references: Vec::new(),
            reference_columns: Box::new(IdentityMapper::new()),
            should_coerce_id: false,
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

    fn converted(&self, key: &str, value: Value, reverse: bool) -> Result<Value> {
        match self.fields.get(key) {
            Some(field) if reverse => field.reverse_convert(&value),
            Some(field) => field.convert(&value),
            None => Ok(value),
        }
    }
}

impl Default for MetaDataRowTypeConverter<Row> {
    fn default() -> Self {
        Self::new()
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
        from.iter()
            .map(|(key, value)| Ok((key.clone(), self.converted(key, value.clone(), false)?)))
            .collect()
    }

    fn reverse_convert(&self, to: &Row) -> Result<Row> {
        to.iter()
            .map(|(key, value)| {
                let key = if self.references.iter().any(|name| name == key) {
                    Mapper::map(self.reference_columns.as_ref(), key.clone())
                } else {
                    key.clone()
                };
                Ok((key.clone(), self.converted(&key, value.clone(), true)?))
            })
            .collect()
    }
}

impl<T: SchemaObject> TypeConverter<T, Row> for MetaDataRowTypeConverter<T> {
    fn convert(&self, from: &Row) -> Result<T> {
        let mut partial = Partial::alloc::<T>().map_err(reflect)?;
        for (index, field) in struct_fields(T::SHAPE)?.iter().enumerate() {
            let cell = from
                .get(field.name)
                .cloned()
                .map(|value| self.converted(field.name, value, false))
                .transpose()?;
            partial = match (is_column(field.shape()), cell.as_ref()) {
                (true, Some(cell)) if *cell != Value::Null => {
                    set_field(partial, field.name, cell, field.shape(), self.should_coerce_id)?
                }
                (true, _) if field.shape().type_identifier.contains("DateTime") => set_field(
                    partial,
                    field.name,
                    &Value::DateTime(chrono::DateTime::<chrono::Utc>::default()),
                    field.shape(),
                    self.should_coerce_id,
                )?,
                _ => partial.set_nth_field_to_default(index).map_err(reflect)?,
            };
        }
        partial.build().map_err(reflect)?.materialize().map_err(reflect)
    }

    fn reverse_convert(&self, to: &T) -> Result<Row> {
        let mut row = Row::new();
        for (field, value) in Peek::new(to).into_struct().map_err(reflect)?.fields() {
            if !is_column(field.shape()) {
                continue;
            }
            if let Some(cell) = peek_value(value) {
                row.insert(
                    field.name.to_string(),
                    self.converted(field.name, cell, true)?,
                );
            }
        }
        Ok(row)
    }
}

pub(crate) fn set_field<'a>(
    partial: Partial<'a>,
    name: &str,
    cell: &Value,
    shape: &'static Shape,
    coerce_id: bool,
) -> Result<Partial<'a>> {
    write_field(partial, name, cell, shape, coerce_id, false)
}

pub(crate) fn set_changed_field<'a>(
    partial: Partial<'a>,
    name: &str,
    cell: &Value,
    shape: &'static Shape,
) -> Result<Partial<'a>> {
    write_field(partial, name, cell, shape, true, true)
}

fn write_field<'a>(
    partial: Partial<'a>,
    name: &str,
    cell: &Value,
    shape: &'static Shape,
    coerce_id: bool,
    change: bool,
) -> Result<Partial<'a>> {
    let partial = partial.begin_field(name).map_err(reflect)?;
    let assign = if change { change_value } else { set_value };
    let partial = if is_option(shape) {
        assign(
            partial.begin_some().map_err(reflect)?,
            cell,
            option_inner(shape)?,
            coerce_id,
        )?
        .end()
        .map_err(reflect)?
    } else {
        assign(partial, cell, shape, coerce_id)?
    };
    partial.end().map_err(reflect)
}

pub(crate) fn set_value<'a>(
    partial: Partial<'a>,
    cell: &Value,
    shape: &'static Shape,
    coerce_id: bool,
) -> Result<Partial<'a>> {
    let cell = match assignable(cell, shape, coerce_id) {
        true => cell.clone(),
        false => match coerced(cell, shape, coerce_id) {
            Some(cell) => cell,
            None => return partial.set_default().map_err(reflect),
        },
    };
    write_value(partial, &cell, shape, coerce_id)
}

pub(crate) fn change_value<'a>(
    partial: Partial<'a>,
    cell: &Value,
    shape: &'static Shape,
    coerce_id: bool,
) -> Result<Partial<'a>> {
    let cell = match assignable(cell, shape, coerce_id) {
        true => cell.clone(),
        false => changed(cell, shape, coerce_id).ok_or_else(|| {
            DatasourceError::message(format!("Cannot assign {cell:?} to {}", shape.type_identifier))
        })?,
    };
    write_value(partial, &cell, shape, coerce_id)
}

fn write_value<'a>(
    partial: Partial<'a>,
    cell: &Value,
    shape: &'static Shape,
    coerce_id: bool,
) -> Result<Partial<'a>> {
    let set = match cell {
        Value::Bool(flag) => partial.set(*flag),
        Value::Int(n) if coerce_id && ScalarType::try_from_shape(shape) == Some(ScalarType::I32) => {
            partial.set(*n as i32)
        }
        Value::Int(n) if ScalarType::try_from_shape(shape) == Some(ScalarType::U32) => {
            partial.set(*n as u32)
        }
        Value::Int(n) => partial.set(*n),
        Value::Float(n) => partial.set(*n),
        Value::Text(text) => partial.set(text.clone()),
        Value::DateTime(date) => partial.set(*date),
        Value::Uuid(id) => partial.set(*id),
        Value::Decimal(number) => partial.set(*number),
        Value::Binary(bytes) => partial.set(bytes.clone()),
        Value::Null if shape.type_identifier.contains("DateTime") => {
            partial.set(chrono::DateTime::<chrono::Utc>::default())
        }
        Value::Null => partial.set_default(),
    };
    set.map_err(reflect)
}

fn assignable(cell: &Value, shape: &'static Shape, coerce_id: bool) -> bool {
    match (cell, ScalarType::try_from_shape(shape)) {
        (Value::Bool(_), Some(ScalarType::Bool)) => true,
        (Value::Int(_), Some(ScalarType::I64)) => true,
        (Value::Int(_), Some(ScalarType::I32)) if coerce_id => true,
        (Value::Int(_), Some(ScalarType::U32)) => true,
        (Value::Float(_), Some(ScalarType::F64)) => true,
        (Value::Text(_), Some(ScalarType::String)) => true,
        (Value::DateTime(_), None) => true,
        (Value::Uuid(_), None) => true,
        (Value::Decimal(_), None) => true,
        (Value::Binary(_), None) => true,
        (Value::Null, _) => true,
        _ => false,
    }
}

fn coerced(cell: &Value, shape: &'static Shape, coerce_id: bool) -> Option<Value> {
    if shape.type_identifier.contains("DateTime") {
        return match cell {
            Value::DateTime(date) => Some(Value::DateTime(*date)),
            Value::Text(text) => chrono::DateTime::parse_from_rfc3339(text)
                .ok()
                .map(|date| Value::DateTime(date.with_timezone(&chrono::Utc))),
            _ => None,
        };
    }
    match (cell, ScalarType::try_from_shape(shape)?) {
        (Value::Int(n), ScalarType::I32) if coerce_id => Some(Value::Int(*n)),
        (Value::Int(n), ScalarType::I64) => Some(Value::Int(*n)),
        (Value::Uuid(id), ScalarType::String) => Some(Value::from(id.to_string())),
        (Value::Text(text), _) if shape.type_identifier.contains("Uuid") => {
            text.parse().ok().map(Value::Uuid)
        }
        (Value::Text(text), _) if shape.type_identifier.contains("Decimal") => {
            text.parse().ok().map(Value::Decimal)
        }
        (Value::Int(n), _) if shape.type_identifier.contains("Decimal") => {
            Some(Value::Decimal(rust_decimal::Decimal::from(*n)))
        }
        _ => None,
    }
}

fn changed(cell: &Value, shape: &'static Shape, coerce_id: bool) -> Option<Value> {
    if let Some(cell) = coerced(cell, shape, coerce_id) {
        return Some(cell);
    }
    match (cell, ScalarType::try_from_shape(shape)?) {
        (Value::Int(n), ScalarType::String) => Some(Value::from(n.to_string())),
        (Value::Bool(flag), ScalarType::String) => Some(Value::from(flag.to_string())),
        (Value::Float(n), ScalarType::String) => Some(Value::from(n.to_string())),
        _ => None,
    }
}

pub(crate) fn peek_value(value: Peek<'_, '_>) -> Option<Value> {
    if let Ok(option) = value.into_option() {
        return match option.value() {
            Some(inner) => peek_value(inner),
            None => Some(Value::Null),
        };
    }
    value
        .get::<bool>()
        .ok()
        .map(|flag| Value::from(*flag))
        .or_else(|| value.get::<i32>().ok().map(|n| Value::from(*n)))
        .or_else(|| value.get::<u32>().ok().map(|n| Value::from(*n as i64)))
        .or_else(|| value.get::<i64>().ok().map(|n| Value::from(*n)))
        .or_else(|| value.get::<f64>().ok().map(|n| Value::from(*n)))
        .or_else(|| value.get::<String>().ok().map(|text| Value::from(text.clone())))
        .or_else(|| {
            value
                .get::<chrono::DateTime<chrono::Utc>>()
                .ok()
                .map(|date| Value::from(*date))
        })
        .or_else(|| value.get::<uuid::Uuid>().ok().map(|id| Value::from(*id)))
        .or_else(|| {
            value
                .get::<rust_decimal::Decimal>()
                .ok()
                .map(|number| Value::from(*number))
        })
        .or_else(|| value.get::<Vec<u8>>().ok().map(|bytes| Value::from(bytes.clone())))
}

pub(crate) fn struct_fields(shape: &'static Shape) -> Result<&'static [Field]> {
    match shape.ty {
        Type::User(UserType::Struct(st)) => Ok(st.fields),
        _ => Err(DatasourceError::message(format!("{} is not a struct", shape.type_identifier))),
    }
}

pub(crate) fn is_column(shape: &'static Shape) -> bool {
    let shape = if is_option(shape) {
        match option_inner(shape) {
            Ok(inner) => inner,
            Err(_) => return false,
        }
    } else {
        shape
    };
    match shape.def {
        Def::Map(_) => false,
        Def::List(list) => list.t.type_identifier == "u8",
        _ => !matches!(shape.ty, Type::User(UserType::Struct(_))),
    }
}

fn is_option(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::Option(_))
}

fn option_inner(shape: &'static Shape) -> Result<&'static Shape> {
    match shape.def {
        Def::Option(option) => Ok(option.t()),
        _ => Err(DatasourceError::message(format!("{} is not an option", shape.type_identifier))),
    }
}

pub(crate) fn reflect(error: impl std::fmt::Display) -> DatasourceError {
    DatasourceError::message(error.to_string())
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
    fn copies_every_cell_on_a_row() {
        let converter = MetaDataRowTypeConverter::new();
        let row = record(&[
            ("id", Value::Int(1)),
            ("name", Value::from("ada")),
            ("extra", Value::from("no")),
        ]);
        assert_eq!(converter.convert(&row).unwrap(), row);
    }

    struct Stamp;

    impl TypeConverter<Value, Value> for Stamp {
        fn convert(&self, from: &Value) -> Result<Value> {
            Ok(match from {
                Value::Text(text) => Value::from(text.to_uppercase()),
                other => other.clone(),
            })
        }

        fn reverse_convert(&self, to: &Value) -> Result<Value> {
            Ok(to.clone())
        }
    }

    #[test]
    fn field_converter_rewrites_the_cell() {
        let converter = MetaDataRowTypeConverter::new().with_fields(HashMap::from([(
            "name".into(),
            Box::new(Stamp) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        )]));
        let converted = converter
            .convert(&record(&[("name", Value::from("ada"))]))
            .unwrap();
        assert_eq!(converted.get("name"), Some(&Value::from("ADA")));
    }
}
