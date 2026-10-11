use bytes::{BufMut, BytesMut};
use chrono::{DateTime, Utc};
use datasources::Value;
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;
use tokio_postgres::types::{IsNull, ToSql, Type};
use uuid::Uuid;

#[derive(Debug)]
pub struct PgValue(pub Value);

impl ToSql for PgValue {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        encode(&self.0, ty, out)
    }

    fn accepts(_: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

fn encode(
    value: &Value,
    ty: &Type,
    out: &mut BytesMut,
) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
    if matches!(value, Value::Null) {
        return Ok(IsNull::Yes);
    }
    match ty {
        ty if ty == &Type::INT2 => i16::try_from(int_of(value)?)?.to_sql(ty, out),
        ty if ty == &Type::INT4 => i32::try_from(int_of(value)?)?.to_sql(ty, out),
        ty if ty == &Type::INT8 => int_of(value)?.to_sql(ty, out),
        ty if ty == &Type::BOOL => bool_of(value)?.to_sql(ty, out),
        ty if ty == &Type::FLOAT4 => (float_of(value)? as f32).to_sql(ty, out),
        ty if ty == &Type::FLOAT8 => float_of(value)?.to_sql(ty, out),
        ty if ty == &Type::NUMERIC => decimal_of(value)?.to_sql(ty, out),
        ty if ty == &Type::TIMESTAMPTZ => datetime_of(value)?.to_sql(ty, out),
        ty if ty == &Type::TIMESTAMP => datetime_of(value)?.naive_utc().to_sql(ty, out),
        ty if ty == &Type::UNKNOWN => value.wire_text().to_sql(&Type::TEXT, out),
        ty if ty == &Type::UUID => uuid_of(value)?.to_sql(ty, out),
        ty if ty == &Type::BYTEA => binary_of(value)?.to_sql(ty, out),
        ty if ty == &Type::JSONB => {
            out.put_u8(1);
            out.extend_from_slice(value.wire_text().as_bytes());
            Ok(IsNull::No)
        }
        ty if ty == &Type::JSON => {
            out.extend_from_slice(value.wire_text().as_bytes());
            Ok(IsNull::No)
        }
        _ => value.wire_text().to_sql(ty, out),
    }
}

fn int_of(value: &Value) -> Result<i64, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Int(number) => Ok(*number),
        Value::Text(text) => Ok(text.parse()?),
        Value::Decimal(number) => Ok(i64::try_from(*number)?),
        _ => Err(format!("not an int: {}", value.wire_text()).into()),
    }
}

fn bool_of(value: &Value) -> Result<bool, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Bool(flag) => Ok(*flag),
        Value::Int(number) => Ok(*number != 0),
        Value::Text(text) => Ok(text.parse()?),
        _ => Err(format!("not a bool: {}", value.wire_text()).into()),
    }
}

fn float_of(value: &Value) -> Result<f64, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Float(number) => Ok(*number),
        Value::Int(number) => Ok(*number as f64),
        Value::Text(text) => Ok(text.parse()?),
        Value::Decimal(number) => Ok(f64::try_from(*number)?),
        _ => Err(format!("not a float: {}", value.wire_text()).into()),
    }
}

fn decimal_of(value: &Value) -> Result<Decimal, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Decimal(number) => Ok(*number),
        Value::Int(number) => Ok(Decimal::from(*number)),
        Value::Float(number) => Decimal::from_str(&number.to_string())
            .map_err(|err| Box::new(err) as Box<dyn Error + Sync + Send>),
        Value::Text(text) => Ok(text.parse()?),
        _ => Err(format!("not a decimal: {}", value.wire_text()).into()),
    }
}

fn datetime_of(value: &Value) -> Result<DateTime<Utc>, Box<dyn Error + Sync + Send>> {
    match value {
        Value::DateTime(stamp) => Ok(*stamp),
        Value::Text(text) => Ok(DateTime::parse_from_rfc3339(text)?.with_timezone(&Utc)),
        _ => Err(format!("not a datetime: {}", value.wire_text()).into()),
    }
}

fn uuid_of(value: &Value) -> Result<Uuid, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Uuid(id) => Ok(*id),
        Value::Text(text) => Ok(text.parse()?),
        _ => Err(format!("not a uuid: {}", value.wire_text()).into()),
    }
}

fn binary_of(value: &Value) -> Result<Vec<u8>, Box<dyn Error + Sync + Send>> {
    match value {
        Value::Binary(bytes) => Ok(bytes.clone()),
        Value::Text(text) => Ok(base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            text,
        )?),
        _ => Err(format!("not binary: {}", value.wire_text()).into()),
    }
}
