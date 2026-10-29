use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Schema {
    pub columns: HashMap<String, String>,
}

impl Schema {
    pub fn new(columns: HashMap<String, String>) -> Self {
        Self { columns }
    }
}
