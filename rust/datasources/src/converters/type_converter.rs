use crate::converters::binary_field_converter::binary_field_converter;
use crate::converters::boolean_field_converter::boolean_field_converter;
use crate::converters::convert_schema::ConvertSchema;
use crate::converters::date_time_field_converter::date_time_field_converter;
use crate::converters::decimal_field_converter::decimal_field_converter;
use crate::converters::field_converter::FieldConverter;
use crate::converters::identity_field_converter::identity_field_converter;
use crate::converters::provider_types::ProviderTypes;
use crate::converters::type_mapping::TypeMapping;
use crate::converters::uuid_field_converter::uuid_field_converter;
use crate::datasource_error::DatasourceError;
use crate::mappers::DatasourceMapper;
use crate::result::Result;
use std::collections::HashMap;
use std::sync::Mutex;

const HOST: &str = "rust";

static REGISTRY: Mutex<Option<HashMap<String, ProviderTypes>>> = Mutex::new(None);

fn registry() -> std::sync::MutexGuard<'static, Option<HashMap<String, ProviderTypes>>> {
    REGISTRY.lock().expect("type converter registry")
}

pub fn use_provider(source: impl Into<String>, types: ProviderTypes) {
    let mut guard = registry();
    guard.get_or_insert_with(HashMap::new).insert(source.into(), types);
}

pub fn reset() {
    *registry() = None;
}

pub fn mapping<T: ConvertSchema>(source: &str, destination: &str) -> Result<TypeMapping> {
    mapping_with::<T>(
        source,
        destination,
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    )
}

pub fn mapping_with<T: ConvertSchema>(
    source: &str,
    destination: &str,
    schema: HashMap<String, String>,
    converters: HashMap<String, FieldConverter>,
    names: HashMap<String, String>,
) -> Result<TypeMapping> {
    if destination != HOST {
        return Err(DatasourceError::message(format!(
            "TypeConverter destination must be \"{HOST}\""
        )));
    }
    let table = registry()
        .as_ref()
        .and_then(|map| map.get(source))
        .cloned()
        .ok_or_else(|| DatasourceError::message(format!("Provider \"{source}\" is not registered")))?;
    let name_pairs: Vec<(String, String)> = names.into_iter().collect();
    let name_refs: Vec<(&str, &str)> = name_pairs
        .iter()
        .map(|(src, dest)| (src.as_str(), dest.as_str()))
        .collect();
    let names = DatasourceMapper::mapping(
        if table.rename { "sql" } else { HOST },
        HOST,
        &name_refs,
    );
    let mut merged: HashMap<String, String> = T::schema()
        .into_iter()
        .map(|(name, sql)| (name.to_string(), sql.to_string()))
        .collect();
    merged.extend(schema);
    Ok(TypeMapping::new(
        type_name::<T>(),
        source.to_string(),
        HOST.to_string(),
        merged,
        names,
        table,
        converters,
    ))
}

fn type_name<T>() -> String {
    std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .into()
}

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
