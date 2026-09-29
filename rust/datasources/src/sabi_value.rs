use crate::isolation_level::IsolationLevel;
use crate::parameter::Parameter;
use crate::parameter_direction::ParameterDirection;
use crate::response::Response;
use crate::row::Row;
use crate::value::Value;
use abi_stable::std_types::{ROption, RResult, RString, RVec};
use abi_stable::StableAbi;
use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
use uuid::Uuid;

#[repr(C)]
#[derive(StableAbi, Clone, Debug)]
pub struct SabiValue {
    pub kind: u8,
    pub text: RString,
    pub int: i64,
    pub float_bits: u64,
    pub bytes: RVec<u8>,
}

#[repr(C)]
#[derive(StableAbi, Clone, Debug)]
pub struct SabiCell {
    pub name: RString,
    pub value: SabiValue,
}

#[repr(C)]
#[derive(StableAbi, Clone, Debug)]
pub struct SabiParameter {
    pub name: RString,
    pub value: SabiValue,
    pub direction: u8,
}

#[repr(C)]
#[derive(StableAbi, Clone, Debug)]
pub struct SabiResponse {
    pub result: SabiValue,
    pub rows_affected: i64,
    pub parameters: ROption<RVec<SabiParameter>>,
}

const NULL: u8 = 0;
const BOOL: u8 = 1;
const INT: u8 = 2;
const FLOAT: u8 = 3;
const TEXT: u8 = 4;
const DATETIME: u8 = 5;
const UUID_KIND: u8 = 6;
const DECIMAL: u8 = 7;
const BINARY: u8 = 8;

fn empty_value(kind: u8) -> SabiValue {
    SabiValue {
        kind,
        text: RString::new(),
        int: 0,
        float_bits: 0,
        bytes: RVec::new(),
    }
}

pub fn encode_value(value: &Value) -> SabiValue {
    match value {
        Value::Null => empty_value(NULL),
        Value::Bool(flag) => SabiValue {
            int: i64::from(*flag),
            ..empty_value(BOOL)
        },
        Value::Int(number) => SabiValue {
            int: *number,
            ..empty_value(INT)
        },
        Value::Float(number) => SabiValue {
            float_bits: number.to_bits(),
            ..empty_value(FLOAT)
        },
        Value::Text(text) => SabiValue {
            text: text.as_str().into(),
            ..empty_value(TEXT)
        },
        Value::DateTime(date) => SabiValue {
            int: date.timestamp_millis(),
            ..empty_value(DATETIME)
        },
        Value::Uuid(id) => SabiValue {
            text: id.to_string().into(),
            ..empty_value(UUID_KIND)
        },
        Value::Decimal(number) => SabiValue {
            text: number.to_string().into(),
            ..empty_value(DECIMAL)
        },
        Value::Binary(bytes) => SabiValue {
            bytes: bytes.as_slice().into(),
            ..empty_value(BINARY)
        },
    }
}

pub fn decode_value(value: &SabiValue) -> Value {
    match value.kind {
        BOOL => Value::Bool(value.int != 0),
        INT => Value::Int(value.int),
        FLOAT => Value::Float(f64::from_bits(value.float_bits)),
        TEXT => Value::Text(value.text.to_string()),
        DATETIME => Value::DateTime(Utc.timestamp_millis_opt(value.int).single().unwrap_or_default()),
        UUID_KIND => value
            .text
            .parse::<Uuid>()
            .map(Value::Uuid)
            .unwrap_or(Value::Null),
        DECIMAL => value
            .text
            .parse::<Decimal>()
            .map(Value::Decimal)
            .unwrap_or(Value::Null),
        BINARY => Value::Binary(value.bytes.to_vec()),
        _ => Value::Null,
    }
}

pub fn encode_row(row: &Row) -> RVec<SabiCell> {
    row.iter()
        .map(|(name, value)| SabiCell {
            name: name.as_str().into(),
            value: encode_value(value),
        })
        .collect()
}

pub fn decode_row(cells: &RVec<SabiCell>) -> Row {
    cells
        .iter()
        .map(|cell| (cell.name.to_string(), decode_value(&cell.value)))
        .collect()
}

pub fn encode_options(options: Option<&HashMap<String, String>>) -> RVec<SabiCell> {
    options
        .map(|map| {
            map.iter()
                .map(|(name, value)| SabiCell {
                    name: name.as_str().into(),
                    value: SabiValue {
                        text: value.as_str().into(),
                        ..empty_value(TEXT)
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn decode_options(cells: &RVec<SabiCell>) -> HashMap<String, String> {
    cells
        .iter()
        .map(|cell| (cell.name.to_string(), cell.value.text.to_string()))
        .collect()
}

fn direction_tag(direction: ParameterDirection) -> u8 {
    match direction {
        ParameterDirection::Input => 0,
        ParameterDirection::Output => 1,
        ParameterDirection::InputOutput => 2,
        ParameterDirection::ReturnValue => 3,
    }
}

fn direction_of(tag: u8) -> ParameterDirection {
    match tag {
        1 => ParameterDirection::Output,
        2 => ParameterDirection::InputOutput,
        3 => ParameterDirection::ReturnValue,
        _ => ParameterDirection::Input,
    }
}

pub fn encode_parameters(parameters: Option<&[Parameter]>) -> ROption<RVec<SabiParameter>> {
    parameters.map_or(ROption::RNone, |list| {
        ROption::RSome(
            list.iter()
                .map(|parameter| SabiParameter {
                    name: parameter.name.as_str().into(),
                    value: encode_value(&parameter.value),
                    direction: direction_tag(parameter.direction),
                })
                .collect(),
        )
    })
}

pub fn decode_parameters(parameters: &ROption<RVec<SabiParameter>>) -> Option<Vec<Parameter>> {
    match parameters {
        ROption::RNone => None,
        ROption::RSome(list) => Some(
            list.iter()
                .map(|parameter| Parameter {
                    name: parameter.name.to_string(),
                    value: decode_value(&parameter.value),
                    direction: direction_of(parameter.direction),
                })
                .collect(),
        ),
    }
}

pub fn encode_response(response: &Response) -> SabiResponse {
    SabiResponse {
        result: encode_value(&response.result),
        rows_affected: response.rows_affected,
        parameters: encode_parameters(response.parameters.as_deref()),
    }
}

pub fn decode_response(response: &SabiResponse) -> Response {
    Response {
        result: decode_value(&response.result),
        rows_affected: response.rows_affected,
        parameters: decode_parameters(&response.parameters),
    }
}

pub fn isolation_tag(level: IsolationLevel) -> u8 {
    match level {
        IsolationLevel::ReadUncommitted => 0,
        IsolationLevel::ReadCommitted => 1,
        IsolationLevel::RepeatableRead => 2,
        IsolationLevel::Serializable => 3,
    }
}

pub fn isolation_of(tag: u8) -> IsolationLevel {
    match tag {
        0 => IsolationLevel::ReadUncommitted,
        1 => IsolationLevel::ReadCommitted,
        2 => IsolationLevel::RepeatableRead,
        _ => IsolationLevel::Serializable,
    }
}

pub fn lift<T>(result: crate::Result<T>) -> RResult<T, RString> {
    match result {
        Ok(value) => RResult::ROk(value),
        Err(error) => RResult::RErr(error.to_string().into()),
    }
}

pub fn lower<T>(result: RResult<T, RString>) -> crate::Result<T> {
    match result {
        RResult::ROk(value) => Ok(value),
        RResult::RErr(error) => Err(crate::DatasourceError::message(error.to_string())),
    }
}
