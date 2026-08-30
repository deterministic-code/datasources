use crate::converters::binary_field_converter::binary_field_converter;
use crate::converters::boolean_field_converter::boolean_field_converter;
use crate::converters::date_time_field_converter::date_time_field_converter;
use crate::converters::decimal_field_converter::decimal_field_converter;
use crate::converters::field_converter::FieldConverter;
use crate::converters::identity_field_converter::identity_field_converter;
use crate::converters::provider_types::ProviderTypes;
use crate::converters::uuid_field_converter::uuid_field_converter;
use std::collections::HashMap;

pub fn canonical_converters(boolean_storage: &str) -> HashMap<String, FieldConverter> {
    HashMap::from([
        ("TEXT".into(), identity_field_converter()),
        ("INTEGER".into(), identity_field_converter()),
        ("REAL".into(), identity_field_converter()),
        ("NUMERIC".into(), decimal_field_converter(None)),
        ("BOOLEAN".into(), boolean_field_converter(boolean_storage)),
        ("DATETIME".into(), date_time_field_converter()),
        ("BLOB".into(), binary_field_converter()),
        ("UUID".into(), uuid_field_converter()),
    ])
}

pub fn sql_provider_types(
    aliases: HashMap<String, String>,
    rename: bool,
    boolean_storage: &str,
) -> ProviderTypes {
    let mut types = canonical_converters(boolean_storage);
    for (alias, target) in aliases {
        let converter = types
            .get(&target)
            .cloned()
            .unwrap_or_else(identity_field_converter);
        types.insert(alias, converter);
    }
    ProviderTypes { rename, types }
}

pub fn identity_provider_types() -> ProviderTypes {
    ProviderTypes {
        rename: false,
        types: HashMap::new(),
    }
}
