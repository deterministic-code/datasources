use datasources::{TypeConverter, Value};
use std::collections::HashMap;

pub fn types() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    HashMap::new()
}

pub fn register() {}
