use crate::converters::convert_schema::ConvertSchema;
use crate::converters::field_converter::FieldConverter;
use crate::converters::provider_types::ProviderTypes;
use crate::converters::type_mapping::TypeMapping;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use std::collections::HashMap;
use std::sync::Mutex;

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

pub fn table(source: &str) -> Result<ProviderTypes> {
    registry()
        .as_ref()
        .and_then(|map| map.get(source))
        .cloned()
        .ok_or_else(|| DatasourceError::message(format!("Provider \"{source}\" is not registered")))
}

pub fn mapping<T: ConvertSchema>(source: &str) -> Result<TypeMapping> {
    mapping_with::<T>(source, HashMap::new())
}

pub fn mapping_with<T: ConvertSchema>(
    source: &str,
    converters: HashMap<String, FieldConverter>,
) -> Result<TypeMapping> {
    let types = table(source)?;
    let fields: HashMap<String, String> = T::schema()
        .into_iter()
        .map(|(name, sql)| (name.to_string(), sql.to_string()))
        .collect();
    Ok(TypeMapping::new(
        type_name::<T>(),
        source.to_string(),
        fields,
        types,
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
