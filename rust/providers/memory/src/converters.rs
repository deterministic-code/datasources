use datasources::{core_types, ValueConverter};
use std::collections::HashMap;

pub fn types() -> HashMap<String, ValueConverter> {
    core_types()
}

pub fn register() {}
