use datasources::chrono::{DateTime, TimeZone, Utc};
use datasources::rust_decimal::Decimal;
use datasources::uuid::Uuid;

#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct MappingCell {
    pub id: i64,
    pub text_to_string: String,
    pub text_to_integer: i64,
    pub text_to_decimal: Decimal,
    pub text_to_datetime: DateTime<Utc>,
    pub text_to_binary: Vec<u8>,
    pub text_to_guid: Uuid,
    pub int_to_integer: i64,
    pub int_to_float: f64,
    pub int_to_decimal: Decimal,
    pub int_to_boolean: bool,
    pub float_to_integer: i64,
    pub float_to_float: f64,
    pub float_to_decimal: Decimal,
    pub decimal_to_integer: i64,
    pub decimal_to_float: f64,
    pub decimal_to_decimal: Decimal,
    pub bool_to_boolean: bool,
    pub datetime_to_datetime: DateTime<Utc>,
    pub binary_to_binary: Vec<u8>,
    pub binary_to_guid: Uuid,
    pub guid_to_string: String,
    pub guid_to_guid: Uuid,
}

impl Default for MappingCell {
    fn default() -> Self {
        Self {
            id: 0,
            text_to_string: String::new(),
            text_to_integer: 0,
            text_to_decimal: Decimal::ZERO,
            text_to_datetime: Utc.timestamp_opt(0, 0).single().unwrap(),
            text_to_binary: Vec::new(),
            text_to_guid: Uuid::nil(),
            int_to_integer: 0,
            int_to_float: 0.0,
            int_to_decimal: Decimal::ZERO,
            int_to_boolean: false,
            float_to_integer: 0,
            float_to_float: 0.0,
            float_to_decimal: Decimal::ZERO,
            decimal_to_integer: 0,
            decimal_to_float: 0.0,
            decimal_to_decimal: Decimal::ZERO,
            bool_to_boolean: false,
            datetime_to_datetime: Utc.timestamp_opt(0, 0).single().unwrap(),
            binary_to_binary: Vec::new(),
            binary_to_guid: Uuid::nil(),
            guid_to_string: String::new(),
            guid_to_guid: Uuid::nil(),
        }
    }
}

impl datasources::SchemaObject for MappingCell {}
