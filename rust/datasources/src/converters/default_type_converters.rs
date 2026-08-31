use crate::converters::type_converter::TypeConverter;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Mutex;

type Catalog = HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>;
type Factory = fn() -> Catalog;

static REGISTRY: Mutex<Option<HashMap<String, Factory>>> = Mutex::new(None);

fn registry() -> std::sync::MutexGuard<'static, Option<HashMap<String, Factory>>> {
    REGISTRY.lock().expect("DefaultTypeConverters")
}

pub struct DefaultTypeConverters;

impl DefaultTypeConverters {
    pub fn use_provider(provider: &str, factory: Factory) {
        registry()
            .get_or_insert_with(HashMap::new)
            .insert(provider.to_string(), factory);
    }

    pub fn for_provider(provider: &str) -> Catalog {
        registry()
            .as_ref()
            .and_then(|map| map.get(provider).copied())
            .map(|factory| factory())
            .unwrap_or_default()
    }
}
