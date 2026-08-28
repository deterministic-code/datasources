use crate::converters::fail::fail;
use crate::converters::field_converter::FieldConverter;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use crate::value::Value;

pub fn decimal_field_converter(size: Option<(usize, usize)>) -> FieldConverter {
    FieldConverter::new(
        move |value| format_decimal(value, size),
        move |value| format_decimal(value, size),
    )
}

fn format_decimal(value: &Value, size: Option<(usize, usize)>) -> Result<Value> {
    let text = match value {
        Value::Null => return Ok(Value::Null),
        Value::Text(text) => text.clone(),
        Value::Int(n) => n.to_string(),
        Value::Float(n) if n.is_finite() => n.to_string(),
        other => return Err(fail(other, "decimal")),
    };
    let Some((precision, scale)) = size else {
        return Ok(Value::Text(text));
    };
    let (sign, rest) = text
        .strip_prefix('-')
        .map(|rest| ("-", rest))
        .unwrap_or(("", text.as_str()));
    let (whole, frac) = rest.split_once('.').unwrap_or((rest, ""));
    if !whole.chars().all(|ch| ch.is_ascii_digit())
        || !frac.chars().all(|ch| ch.is_ascii_digit())
        || whole.is_empty()
    {
        return Err(fail(value, "decimal"));
    }
    let mut frac = frac.to_string();
    while frac.len() < scale {
        frac.push('0');
    }
    frac.truncate(scale);
    if whole.len() + if scale > 0 { frac.len() } else { 0 } > precision {
        return Err(DatasourceError::message(format!(
            "Decimal {text} exceeds size [{precision}, {scale}]"
        )));
    }
    Ok(Value::Text(if scale > 0 {
        format!("{sign}{whole}.{frac}")
    } else {
        format!("{sign}{whole}")
    }))
}
