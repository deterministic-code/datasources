use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use facet::{Def, Facet, Field, HasFields, Partial, Peek, ScalarType, Shape, Type, UserType};

pub trait SchemaObject: Facet<'static> + Clone + Default + Send + Sync + 'static {}

pub trait FromSchema: Clone + Default + Send + Sync + 'static {
    fn from_schema(row: Row, schema: &Schema) -> Result<Self>;
    fn to_schema(&self, schema: &Schema) -> Result<Row>;
}

impl FromSchema for Row {
    fn from_schema(row: Row, _: &Schema) -> Result<Self> {
        Ok(row)
    }

    fn to_schema(&self, _: &Schema) -> Result<Row> {
        Ok(self.clone())
    }
}

impl<T: SchemaObject> FromSchema for T {
    fn from_schema(row: Row, schema: &Schema) -> Result<Self> {
        read_facet::<T>(row, schema)
    }

    fn to_schema(&self, schema: &Schema) -> Result<Row> {
        write_facet(self, schema)
    }
}

fn reflect(error: impl std::fmt::Display) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

fn read_facet<T: Facet<'static>>(row: Row, schema: &Schema) -> Result<T> {
    let mut partial = Partial::alloc::<T>().map_err(reflect)?;
    let fields = struct_fields(T::SHAPE)?;
    for (index, field) in fields.iter().enumerate() {
        let name = field.name;
        if let Some(_spec) = schema.columns.get(name) {
            partial = match row.get(name) {
                Some(cell) if *cell != Value::Null => set_scalar(partial, name, cell, field.shape())?,
                _ => partial.set_nth_field_to_default(index).map_err(reflect)?,
            };
            continue;
        }
        partial = match field.shape().def {
            Def::List(_) => read_list(partial, name, field.shape(), decode_rows(row.get(name)))?,
            Def::Map(_) => partial
                .begin_field(name)
                .and_then(Partial::init_map)
                .and_then(Partial::end)
                .map_err(reflect)?,
            _ => match row.get(name) {
                Some(cell) if *cell != Value::Null => set_scalar(partial, name, cell, field.shape())?,
                _ => partial.set_nth_field_to_default(index).map_err(reflect)?,
            },
        };
    }
    let value = partial.build().map_err(reflect)?;
    value.materialize::<T>().map_err(reflect)
}

fn write_facet<T: Facet<'static>>(item: &T, schema: &Schema) -> Result<Row> {
    let peek = Peek::new(item);
    let st = peek.into_struct().map_err(reflect)?;
    let mut row = Row::new();
    for (field, value) in st.fields() {
        let name = field.name;
        if let Some(cell) = peek_cell(value) {
            row.insert(name.to_string(), cell);
            continue;
        }
        if matches!(value.shape().def, Def::List(_)) {
            row.insert(name.to_string(), encode_rows(&peek_list_rows(value, schema)?));
        }
    }
    Ok(row)
}

fn read_list<'f>(
    partial: Partial<'f>,
    name: &str,
    shape: &'static Shape,
    rows: Vec<Row>,
) -> Result<Partial<'f>> {
    let element = list_element(shape)?;
    let mut next = partial
        .begin_field(name)
        .and_then(Partial::init_list)
        .map_err(reflect)?;
    for row in rows {
        next = next.begin_list_item().map_err(reflect)?;
        next = fill_shape(next, element, row)?;
        next = next.end().map_err(reflect)?;
    }
    next.end().map_err(reflect)
}

fn fill_shape<'f>(mut partial: Partial<'f>, shape: &'static Shape, row: Row) -> Result<Partial<'f>> {
    let Ok(fields) = struct_fields(shape) else {
        return set_from_value(partial, row.values().next().unwrap_or(&Value::Null), shape);
    };
    for (index, field) in fields.iter().enumerate() {
        partial = match row.get(field.name) {
            Some(cell) if *cell != Value::Null => set_scalar(partial, field.name, cell, field.shape())?,
            _ => partial.set_nth_field_to_default(index).map_err(reflect)?,
        };
    }
    Ok(partial)
}

fn set_scalar<'a>(
    partial: Partial<'a>,
    name: &str,
    cell: &Value,
    shape: &'static Shape,
) -> Result<Partial<'a>> {
    if matches!(shape.def, Def::Option(_)) {
        if *cell == Value::Null {
            return partial
                .begin_field(name)
                .and_then(Partial::set_default)
                .and_then(Partial::end)
                .map_err(reflect);
        }
        return set_from_value(partial.begin_field(name).map_err(reflect)?, cell, option_inner(shape)?)
            .and_then(|next| next.end().map_err(reflect));
    }
    set_from_value(partial.begin_field(name).map_err(reflect)?, cell, shape)
        .and_then(|next| next.end().map_err(reflect))
}

fn set_from_value<'a>(partial: Partial<'a>, cell: &Value, shape: &'static Shape) -> Result<Partial<'a>> {
    let set = match (cell, ScalarType::try_from_shape(shape), shape.type_identifier) {
        (Value::Bool(flag), Some(ScalarType::Bool), _) => partial.set(*flag),
        (Value::Int(n), Some(ScalarType::I8), _) => partial.set(*n as i8),
        (Value::Int(n), Some(ScalarType::I16), _) => partial.set(*n as i16),
        (Value::Int(n), Some(ScalarType::I32), _) => partial.set(*n as i32),
        (Value::Int(n), Some(ScalarType::I64), _) => partial.set(*n),
        (Value::Int(n), Some(ScalarType::U32), _) => partial.set(*n as u32),
        (Value::Float(n), Some(ScalarType::F64), _) => partial.set(*n),
        (Value::Text(text), Some(ScalarType::String), _) => partial.set(text.clone()),
        (Value::Text(text), _, "String") => partial.set(text.clone()),
        (Value::Int(n), _, "i32") => partial.set(*n as i32),
        (Value::Int(n), _, "i64") => partial.set(*n),
        (Value::Binary(bytes), _, _) if matches!(shape.def, Def::List(_)) && list_element(shape).is_ok_and(|inner| inner.type_identifier == "u8") => {
            partial.set(bytes.clone())
        }
        (Value::Null, _, _) => partial.set_default(),
        _ => return Err(DatasourceError::message(format!("cannot set {} from {cell:?}", shape.type_identifier))),
    };
    set.map_err(reflect)
}

fn peek_cell(value: Peek<'_, '_>) -> Option<Value> {
    if let Ok(flag) = value.get::<bool>() {
        return Some(Value::Bool(*flag));
    }
    if let Ok(n) = value.get::<i8>() {
        return Some(Value::Int(*n as i64));
    }
    if let Ok(n) = value.get::<i16>() {
        return Some(Value::Int(*n as i64));
    }
    if let Ok(n) = value.get::<i32>() {
        return Some(Value::Int(*n as i64));
    }
    if let Ok(n) = value.get::<i64>() {
        return Some(Value::Int(*n));
    }
    if let Ok(n) = value.get::<u32>() {
        return Some(Value::Int(*n as i64));
    }
    if let Ok(n) = value.get::<f64>() {
        return Some(Value::Float(*n));
    }
    if let Ok(text) = value.get::<String>() {
        return Some(Value::from(text.clone()));
    }
    if let Ok(bytes) = value.get::<Vec<u8>>() {
        return Some(Value::Binary(bytes.clone()));
    }
    if let Ok(option) = value.into_option() {
        return option.value().and_then(peek_cell);
    }
    None
}

fn peek_list_rows(value: Peek<'_, '_>, _schema: &Schema) -> Result<Vec<Row>> {
    let list = value.into_list_like().map_err(reflect)?;
    let mut rows = Vec::new();
    for item in list.iter() {
        if let Ok(st) = item.into_struct() {
            let mut row = Row::new();
            for (field, cell) in st.fields() {
                if let Some(value) = peek_cell(cell) {
                    row.insert(field.name.to_string(), value);
                }
            }
            rows.push(row);
        }
    }
    Ok(rows)
}

fn struct_fields(shape: &'static Shape) -> Result<&'static [Field]> {
    match shape.ty {
        Type::User(UserType::Struct(st)) => Ok(st.fields),
        _ => Err(DatasourceError::message(format!("{} is not a struct", shape.type_identifier))),
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
